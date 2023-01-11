import * as cyfs from 'cyfs-sdk';
import { DownloadFileTaskState } from './file/download';
import { downloadMgr } from './file/download_mgr';
import { PublishFileTask } from './file/publish';
import {
    AsDecoderFromRaw,
    AsObject,
    fromNONObjectInfo,
    makeBuckyErr,
    toNONObjectInfo
} from './kits';

// Simulator starts with parameter `--simulator ZoneNo DeviceNo`

export enum SimulatorZoneNo {
    REAL = 0,
    FIRST = 1,
    SECOND = 2
}

export enum SimulatorDeviceNo {
    FIRST = 1,
    SECOND = 2
}

const g_simulatorPortOOD = {
    [SimulatorZoneNo.REAL]: undefined,
    [SimulatorZoneNo.FIRST]: {
        http: 21000,
        ws: 21001
    },
    [SimulatorZoneNo.SECOND]: {
        http: 21010,
        ws: 21011
    }
};

const g_simulatorPortRuntime = {
    [SimulatorZoneNo.FIRST]: {
        [SimulatorDeviceNo.FIRST]: {
            http: 21002,
            ws: 21003
        },
        [SimulatorDeviceNo.SECOND]: {
            http: 21004,
            ws: 21005
        }
    },
    [SimulatorZoneNo.SECOND]: {
        [SimulatorDeviceNo.FIRST]: {
            http: 21012,
            ws: 21013
        },
        [SimulatorDeviceNo.SECOND]: {
            http: 21014,
            ws: 21015
        }
    }
};

let g_useSimulator:
    | {
          zoneNo: SimulatorZoneNo;
          deviceNo: SimulatorDeviceNo;
      }
    | undefined;

export function checkSimulator(): [SimulatorZoneNo, SimulatorDeviceNo] {
    if (g_useSimulator) {
        return [g_useSimulator.zoneNo, g_useSimulator.deviceNo];
    }

    const envIndex = process.argv.findIndex((arg) => arg === '--simulator');
    if (envIndex >= 0) {
        let deviceNo = SimulatorDeviceNo.FIRST;
        if (process.argv[envIndex + 2] === '2') {
            deviceNo = SimulatorDeviceNo.SECOND;
        }
        if (process.argv[envIndex + 1] === '2') {
            return [SimulatorZoneNo.SECOND, deviceNo];
        } else if (process.argv[envIndex + 1] === '1') {
            return [SimulatorZoneNo.FIRST, deviceNo];
        }
        return [SimulatorZoneNo.FIRST, SimulatorDeviceNo.FIRST];
    }
    return [SimulatorZoneNo.REAL, 0];
}

// checkXXX，调用方自行保证协议栈上线，是可用状态
// 一般都是APP启动时候等待一次上线，后面就可以直接checkXXX

// 协议栈初始化
export class StackWraper {
    public static wrap(stack: cyfs.SharedCyfsStack, decId?: cyfs.ObjectId): StackWraper {
        return new StackWraper(stack, decId);
    }

    protected m_stack: cyfs.SharedCyfsStack;
    protected m_decId?: cyfs.ObjectId;
    public constructor(stack: cyfs.SharedCyfsStack, decId?: cyfs.ObjectId) {
        this.m_stack = stack;
        this.m_decId = decId;
    }

    public check(): cyfs.SharedCyfsStack {
        return this.m_stack;
    }

    public get decId(): cyfs.ObjectId | undefined {
        return this.m_decId;
    }

    public checkOwner(): cyfs.ObjectId {
        return this.m_stack.local_device().desc().owner()!;
    }

    public checkOwnerPeopleId(): cyfs.PeopleId {
        return cyfs.PeopleId.try_from_object_id(this.checkOwner()).unwrap();
    }

    public async getObject<O extends AsObject>(
        objId: cyfs.ObjectId,
        decoderGen: new () => AsDecoderFromRaw<O>,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            level?: cyfs.NONAPILevel;
            flags?: number;
            target?: cyfs.ObjectId;
            innerPath?: string;
        }
    ): Promise<cyfs.BuckyResult<{ object: O; updateTime?: number; expiresTime?: number }>> {
        const r = await this.m_stack.non_service().get_object({
            object_id: objId,
            inner_path: options?.innerPath,
            common: {
                req_path: options?.reqPath,
                dec_id: options?.decId,
                level: options?.level || cyfs.NONAPILevel.Router,
                flags: options?.flags || 0,
                target: options?.target
            }
        });
        if (r.err) {
            console.error(`get object failed, ${r}`);
            return r;
        }
        const resp = r.unwrap();
        return fromNONObjectInfo(resp.object, decoderGen).map((object) => ({
            object,
            updateTime:
                resp.object_update_time && cyfs.bucky_time_2_js_time(resp.object_update_time),
            expiresTime:
                resp.object_expires_time && cyfs.bucky_time_2_js_time(resp.object_expires_time)
        }));
    }

    public async getObjectNon(
        objId: cyfs.ObjectId,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            level?: cyfs.NONAPILevel;
            flags?: number;
            target?: cyfs.ObjectId;
            innerPath?: string;
        }
    ): Promise<cyfs.BuckyResult<cyfs.NONGetObjectOutputResponse>> {
        return await this.m_stack.non_service().get_object({
            object_id: objId,
            inner_path: options?.innerPath,
            common: {
                req_path: options?.reqPath,
                dec_id: options?.decId,
                level: options?.level || cyfs.NONAPILevel.Router,
                flags: options?.flags || 0,
                target: options?.target
            }
        });
    }

    public async postObject<O extends AsObject>(
        obj: AsObject,
        decoderGen: new () => AsDecoderFromRaw<O>,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            level?: cyfs.NONAPILevel;
            flags?: number;
            target?: cyfs.ObjectId;
        }
    ): Promise<cyfs.BuckyResult<O | undefined>> {
        return await this.postEncodedObject(toNONObjectInfo(obj), decoderGen, options);
    }

    public async postObjectNoDecode(
        obj: AsObject,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            level?: cyfs.NONAPILevel;
            flags?: number;
            target?: cyfs.ObjectId;
        }
    ): Promise<cyfs.BuckyResult<cyfs.NONObjectInfo | undefined>> {
        return await this.postEncodedObjectNoDecode(toNONObjectInfo(obj), options);
    }

    public async postEncodedObject<O extends AsObject>(
        encodedObj: cyfs.NONObjectInfo,
        decoderGen: new () => AsDecoderFromRaw<O>,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            level?: cyfs.NONAPILevel;
            flags?: number;
            target?: cyfs.ObjectId;
        }
    ): Promise<cyfs.BuckyResult<O | undefined>> {
        const r = await this.postEncodedObjectNoDecode(encodedObj, options);

        if (r.err) {
            console.error(`post object failed, ${r}`);
            return r;
        }
        const resp = r.unwrap();
        if (resp) {
            return fromNONObjectInfo(resp, decoderGen);
        } else {
            return cyfs.Ok(undefined);
        }
    }

    public async postEncodedObjectNoDecode(
        encodedObj: cyfs.NONObjectInfo,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            level?: cyfs.NONAPILevel;
            flags?: number;
            target?: cyfs.ObjectId;
        }
    ): Promise<cyfs.BuckyResult<cyfs.NONObjectInfo | undefined>> {
        const r = await this.m_stack.non_service().post_object({
            common: {
                req_path: options?.reqPath,
                dec_id: options?.decId,
                level: options?.level || cyfs.NONAPILevel.Router,
                flags: options?.flags || 0,
                target: options?.target
            },
            object: encodedObj
        });

        if (r.err) {
            console.error(`from unknown zone, ${r}`);
            return r;
        }
        const resp = r.unwrap();
        return cyfs.Ok(resp.object);
    }

    public async putObject(
        obj: AsObject,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            level?: cyfs.NONAPILevel;
            flags?: number;
            target?: cyfs.ObjectId;
        }
    ): Promise<cyfs.BuckyResult<cyfs.NONPutObjectOutputResponse>> {
        return await this.putEncodedObject(toNONObjectInfo(obj), options);
    }

    public async putEncodedObject(
        obj: cyfs.NONObjectInfo,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            level?: cyfs.NONAPILevel;
            flags?: number;
            target?: cyfs.ObjectId;
        }
    ): Promise<cyfs.BuckyResult<cyfs.NONPutObjectOutputResponse>> {
        return await this.m_stack.non_service().put_object({
            common: {
                req_path: options?.reqPath,
                dec_id: options?.decId,
                level: options?.level || cyfs.NONAPILevel.Router,
                flags: options?.flags || 0,
                target: options?.target
            },
            object: obj
        });
    }

    public async removeObjectNoDecode(
        objectId: cyfs.ObjectId,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            level?: cyfs.NONAPILevel;
            flags?: number;
            target?: cyfs.ObjectId;
            innerPath?: string;
        }
    ): Promise<cyfs.BuckyResult<cyfs.NONObjectInfo | undefined>> {
        const r = await this.m_stack.non_service().delete_object({
            common: {
                req_path: options?.reqPath,
                dec_id: options?.decId,
                level: options?.level || cyfs.NONAPILevel.Router,
                flags: options?.flags || 0,
                target: options?.target
            },
            object_id: objectId,
            inner_path: options?.innerPath
        });

        return r.map((resp) => resp.object);
    }

    public async signObject<O extends AsObject>(
        obj: AsObject,
        decoderGen: new () => AsDecoderFromRaw<O>,
        options?: {
            reqPath?: string;
            decId?: cyfs.ObjectId;
            flags?: number;
            target?: cyfs.ObjectId;
            isSignDesc: boolean;
            isAddSign: boolean;
            usePeople: boolean;
        }
    ): Promise<cyfs.BuckyResult<{ object?: O; state: 'signed' | 'pending' }>> {
        let flags = cyfs.CRYPTO_REQUEST_FLAG_SIGN_BY_DEVICE;
        if (options?.usePeople) {
            flags = cyfs.CRYPTO_REQUEST_FLAG_SIGN_BY_PEOPLE;
        }

        if (options?.isAddSign) {
            if (options?.isSignDesc) {
                flags |= cyfs.CRYPTO_REQUEST_FLAG_SIGN_PUSH_DESC;
            } else {
                flags |= cyfs.CRYPTO_REQUEST_FLAG_SIGN_PUSH_BODY;
            }
        } else {
            if (options?.isSignDesc) {
                flags |= cyfs.CRYPTO_REQUEST_FLAG_SIGN_SET_DESC;
            } else {
                flags |= cyfs.CRYPTO_REQUEST_FLAG_SIGN_SET_BODY;
            }
        }

        const r = await this.m_stack.crypto().sign_object({
            common: {
                req_path: options?.reqPath,
                dec_id: options?.decId,
                flags: options?.flags || 0,
                target: options?.target
            },
            object: toNONObjectInfo(obj),
            flags
        });

        if (r.err) {
            console.error(`sign object failed, ${r}`);
            return r;
        }

        const resp = r.unwrap();
        if (!resp.object) {
            return cyfs.Ok({ state: resp.result });
        }
        const dr = fromNONObjectInfo(resp.object, decoderGen);
        if (dr.err) {
            console.error(`sign object failed when decode, ${dr}`);
            return dr;
        }
        return dr.map((obj) => ({ object: obj, state: resp.result }));
    }

    public async verifyObject(
        obj: AsObject,
        signSource: cyfs.VerifyObjectType,
        options?: {
            signType?: cyfs.VerifySignType;
            reqPath?: string;
            decId?: cyfs.ObjectId;
            flags?: number;
            target?: cyfs.ObjectId;
        }
    ): Promise<cyfs.BuckyResult<boolean>> {
        {
            let signsWraper: Array<cyfs.Signature> | undefined;
            if (options?.signType === cyfs.VerifySignType.Body) {
                signsWraper = obj.signs().body_signs();
            } else if (options?.signType === cyfs.VerifySignType.Desc) {
                signsWraper = obj.signs().desc_signs();
            } else if (options?.signType === cyfs.VerifySignType.Both) {
                let signsArray: Array<cyfs.Signature> = [];
                const signsWraperDesc = obj.signs().desc_signs();
                const signsWraperBody = obj.signs().body_signs();
                if (signsWraperDesc) {
                    signsArray = signsWraperDesc;
                }
                if (signsWraperBody) {
                    signsArray.push(...signsWraperBody);
                }
                signsWraper = signsArray;
            } else {
                const msg = `unknow VerifySignType(${options?.signType})`;
                console.error(msg);
                return makeBuckyErr(cyfs.BuckyErrorCode.Unmatch, msg);
            }

            if (!signsWraper || signsWraper.length === 0) {
                const msg = 'not sign.';
                return makeBuckyErr(cyfs.BuckyErrorCode.InvalidSignature, msg);
            }
        }

        const verifyR = await this.m_stack.crypto().verify_object({
            common: {
                req_path: options?.reqPath,
                dec_id: options?.decId,
                flags: options?.flags || 0,
                target: options?.target
            },
            object: toNONObjectInfo(obj),
            sign_type: options?.signType || cyfs.VerifySignType.Desc,
            sign_object: signSource
        });

        return verifyR.map((r) => r.result.valid);
    }

    public async getDeviceFromCache(
        deviceId: cyfs.DeviceId
    ): Promise<cyfs.BuckyResult<cyfs.Device>> {
        const r = await this.getObject(deviceId.object_id, cyfs.DeviceDecoder, {
            level: cyfs.NONAPILevel.NOC
        });
        return r.map((r) => r.object);
    }

    public async isReqestInZone(
        req: cyfs.RouterHandlerPostObjectRequest
    ): Promise<cyfs.BuckyResult<boolean>> {
        const devR = await this.getDeviceFromCache(req.request.common.source.zone.device!);
        if (devR.err) {
            const msg = `get source device failed, ${devR}`;
            console.error(msg);
            return devR;
        }
        const sourceDevice = devR.unwrap();
        const sourceOwner = sourceDevice.desc().owner();
        if (!sourceOwner?.eq(this.checkOwner())) {
            return cyfs.Ok(false);
        }
        return cyfs.Ok(true);
    }

    public async getOODByOwner(
        ownerId: cyfs.ObjectId
    ): Promise<cyfs.BuckyResult<Array<cyfs.DeviceId>>> {
        const rsR = await this.m_stack.util().resolve_ood({
            common: { flags: 0 },
            object_id: ownerId,
            owner_id: ownerId
        });
        return rsR.map((resp) => resp.device_list);
    }

    public async publishFile(
        filePath: string,
        decId: cyfs.ObjectId,
        chunkSize: number = 4 << 20
    ): Promise<cyfs.BuckyResult<cyfs.FileId>> {
        const task = new PublishFileTask(filePath, { stack: this.m_stack, chunkSize, decId });
        return await task.publish();
    }

    public async downloadFile(
        fileId: cyfs.FileId,
        savePath: string,
        options: {
            owner?: cyfs.PeopleId;
            mainDeviceIdOrTaskId?: cyfs.DeviceId | string; // 续传任务填TaskId(string)
            accDeviceList: Array<cyfs.DeviceId>;
            onTotal?: (totalSize: number) => void;
            onProgress?: (percent: number) => boolean; // 返回true表示要停止[abort]
            decId: cyfs.ObjectId;
        }
    ): Promise<cyfs.BuckyResult<{ isAbort: boolean }>> {
        let resolve: (result: cyfs.BuckyResult<{ isAbort: boolean }>) => void;
        const waiter = new Promise<cyfs.BuckyResult<{ isAbort: boolean }>>((_resolve) => {
            resolve = _resolve;
        });

        const { task } = downloadMgr!.createTask(fileId, savePath, {
            ...options
        });

        let handleProgress: undefined | (() => void);
        let handleTotal: undefined | (() => void);
        if (options.onProgress) {
            handleProgress = () => {
                if (options.onProgress!(task.percent)) {
                    task.stop();
                }
            };
            task.on('progress', handleProgress);
        }
        if (options.onTotal) {
            handleTotal = () => {
                options.onTotal!(task.total);
            };
            task.on('total', handleTotal);
        }

        const postEnd = () => {
            task.removeListener('state', handleEnd);
            if (handleProgress) {
                task.removeListener('progress', handleProgress);
            }
            if (handleTotal) {
                task.removeListener('total', handleTotal);
            }
            task.destroy();
        };

        const handleEnd = () => {
            switch (task.state) {
                case DownloadFileTaskState.STANDBY:
                case DownloadFileTaskState.START:
                    break;
                case DownloadFileTaskState.FAIL:
                    resolve(cyfs.Err(task.err!));
                    postEnd();
                    break;
                case DownloadFileTaskState.STOP:
                    resolve(cyfs.Ok({ isAbort: true }));
                    postEnd();
                    break;
                case DownloadFileTaskState.SUCCESS:
                    resolve(cyfs.Ok({ isAbort: false }));
                    postEnd();
                    break;
                default:
                    console.error(`[downloadfile] should not reach here, err state(${task.state})`);
            }
        };
        task.on('state', handleEnd);

        const startR = await task.start();
        if (startR.err) {
            resolve!(cyfs.Err(task.err!));
            postEnd();
        }

        return await waiter;
    }

    public async waitOnline(): Promise<cyfs.BuckyResult<void>> {
        // 等待节点上线
        console.info('will wait online.');
        while (true) {
            const r = await this.m_stack.wait_online(cyfs.JSBI.BigInt(1000000));
            if (r.err) {
                console.error(`wait online err: ${r.val}`);
            } else {
                console.info('online success.');
                console.info(`device: ${this.m_stack.local_device_id()}`);
                console.info(`owner: ${this.m_stack.local_device().desc().owner()}`);
                break;
            }
        }
        return cyfs.Ok(undefined);
    }
}

// OOD端的`DEC-Service`初始化
class StackInitializerOOD extends StackWraper {
    public constructor(decId: cyfs.ObjectId) {
        console.info(`will open stack, ${JSON.stringify(process.argv)}.`);
        let stack: cyfs.SharedCyfsStack;
        const simPort = g_simulatorPortOOD[checkSimulator()[0]];
        if (simPort) {
            const param = cyfs.SharedCyfsStackParam.new_with_ws_event_ports(
                simPort.http,
                simPort.ws,
                decId
            );
            if (param.err) {
                console.error(`init SharedCyfsStackParam failed, ${param}`);
            }
            stack = cyfs.SharedCyfsStack.open(param.unwrap());
        } else {
            stack = cyfs.SharedCyfsStack.open_default(decId);
        }

        super(stack, decId);
    }
}

// 客户端的`Runtime`初始化
class StackInitializerRuntime extends StackWraper {
    public constructor(decId: cyfs.ObjectId) {
        console.info('will open stack runtime.');
        let stack: cyfs.SharedCyfsStack;
        const [simNo, devNo] = checkSimulator();
        if (simNo !== SimulatorZoneNo.REAL) {
            const simPort = g_simulatorPortRuntime[simNo][devNo!];
            const param = cyfs.SharedCyfsStackParam.new_with_ws_event_ports(
                simPort.http,
                simPort.ws,
                decId
            );
            stack = cyfs.SharedCyfsStack.open(param.unwrap());
        } else {
            stack = cyfs.SharedCyfsStack.open_runtime(decId);
        }

        super(stack, decId);
    }
}

let g_stack: StackWraper | undefined;

export function useSimulator(zoneNo: SimulatorZoneNo, deviceNo: SimulatorDeviceNo) {
    g_useSimulator = { zoneNo, deviceNo };
}

export async function waitStackOOD(decId: cyfs.ObjectId): Promise<cyfs.BuckyResult<StackWraper>> {
    if (!g_stack) {
        g_stack = new StackInitializerOOD(decId);
    }
    return (await g_stack.waitOnline()).map((_) => g_stack!);
}

export async function waitStackRuntime(
    decId: cyfs.ObjectId
): Promise<cyfs.BuckyResult<StackWraper>> {
    if (!g_stack) {
        g_stack = new StackInitializerRuntime(decId);
    }
    return (await g_stack.waitOnline()).map((_) => g_stack!);
}

export async function initWithNativeStack(
    stack: cyfs.SharedCyfsStack
): Promise<cyfs.BuckyResult<StackWraper>> {
    if (!g_stack) {
        g_stack = StackWraper.wrap(stack);
    }
    return cyfs.Ok(g_stack);
}

export function checkStack(): StackWraper {
    if (!g_stack) {
        console.error('the stack has not been init.');
    }
    return g_stack!;
}
