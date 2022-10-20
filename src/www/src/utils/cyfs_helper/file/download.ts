import { EventEmitter } from 'events';
import * as path from 'path';
import * as cyfs from 'cyfs-sdk';
import { makeBuckyErr } from '../kits';

/**
 * TODO: Use the owner of the file itself as the acceleration node
 */
export enum DownloadFileTaskState {
    STANDBY = 0,
    START = 1,
    STOP = 2,
    SUCCESS = 3,
    FAIL = 4,
    DELETE = 5
}

export interface DownloadFileTaskEvent {
    on: ((event: 'state', listener: () => void) => this) & // File upload/download task status change
        ((event: 'progress', listener: () => void) => this) & // File upload/download progress changes
        ((event: 'name', listener: () => void) => this) & // name change
        ((event: 'total', listener: () => void) => this); // size change
}

export class DownloadFileTask extends EventEmitter implements DownloadFileTaskEvent {
    private m_fileId: cyfs.FileId;
    private m_savePath: string;
    private m_stack: cyfs.SharedCyfsStack;
    private m_accDeviceList: cyfs.DeviceId[];
    private m_state: DownloadFileTaskState;
    private m_name: string;
    private m_total: number;
    private m_percent: number;
    private m_speed: number;
    private m_taskId: string | undefined;
    private m_file: cyfs.File | undefined;
    private m_owner: cyfs.PeopleId | undefined;
    private m_mainSource: cyfs.DeviceId | undefined;
    private m_ownerOOD: cyfs.DeviceId[];
    private m_err?: cyfs.BuckyError;
    private m_decId: cyfs.ObjectId;

    public constructor(
        fileId: cyfs.FileId,
        savePath: string,
        options: {
            stack: cyfs.SharedCyfsStack;
            owner?: cyfs.PeopleId;
            mainDeviceIdOrTaskId?: cyfs.DeviceId | string; // Fill in TaskId(string) for the resumed task
            accDeviceList: cyfs.DeviceId[];
            decId: cyfs.ObjectId;
        }
    ) {
        super();
        this.m_fileId = fileId;
        this.m_savePath = savePath;
        this.m_stack = options.stack;
        this.m_state = DownloadFileTaskState.STANDBY;
        this.m_name = path.basename(savePath);
        this.m_total = 0;
        this.m_accDeviceList = [...options.accDeviceList];
        this.m_owner = options.owner;
        this.m_ownerOOD = [];
        if (options.mainDeviceIdOrTaskId) {
            if (typeof options.mainDeviceIdOrTaskId === 'string') {
                this.m_taskId = options.mainDeviceIdOrTaskId;
            } else {
                this.m_mainSource = options.mainDeviceIdOrTaskId;
            }
        }

        this.m_percent = 0;
        this.m_speed = 0;
        this.m_decId = options.decId;

        console.info(`download task(${this.m_fileId.to_base_58()}|${this.m_taskId}}) create.`);
    }

    public get name(): string {
        return this.m_name;
    }
    public get total(): number {
        return this.m_total;
    }
    public get percent(): number {
        return this.m_percent;
    }
    public get speed(): number {
        return this.m_state === DownloadFileTaskState.START ? this.m_speed : 0;
    }
    public get state(): DownloadFileTaskState {
        return this.m_state;
    }
    public get err(): cyfs.BuckyError | undefined {
        return this.m_err;
    }

    public get file(): cyfs.File | undefined {
        return this.m_file;
    }

    public get taskId(): string | undefined {
        return this.m_taskId;
    }

    public async start(): Promise<cyfs.BuckyResult<void>> {
        if (this.m_state === DownloadFileTaskState.START) {
            console.warn(
                `download download task(${this.m_fileId.to_base_58()}|${
                    this.m_taskId
                }) is starting.`
            );
            return cyfs.Ok(undefined);
        } else if (this.m_state === DownloadFileTaskState.SUCCESS) {
            console.warn(
                `download download task(${this.m_fileId.to_base_58()}|${
                    this.m_taskId
                }) has successed.`
            );
            return cyfs.Ok(undefined);
        } else if (this.m_state === DownloadFileTaskState.DELETE) {
            console.error(
                `download download task(${this.m_fileId.to_base_58()}|${
                    this.m_taskId
                }) has deleted.`
            );
            return makeBuckyErr(cyfs.BuckyErrorCode.ErrorState, `err state:${this.m_state}`);
        }

        // 重新开始
        this.m_state = DownloadFileTaskState.STANDBY;

        const changeFailed = (err: cyfs.BuckyError) => {
            this.m_state = DownloadFileTaskState.FAIL;
            this.m_err = err;
            this.emit('state');
        };

        const loadFileR = await this.queryFileObject();
        if (loadFileR.err) {
            changeFailed(loadFileR.val);
            return loadFileR;
        }
        const oodR = await this._getOwnerOOD();
        if (oodR.err) {
            changeFailed(oodR.val);
            return oodR;
        }

        if (!this.m_taskId) {
            const accDeviceList = [...this.m_accDeviceList, ...this.m_ownerOOD];
            if (this.m_mainSource) {
                accDeviceList.unshift(this.m_mainSource);
            }

            console.info(
                `download task(${this.m_fileId.to_base_58()}|${
                    this.m_taskId
                }) will create_task, device_list=${JSON.stringify(accDeviceList)}.`
            );

            const cr = await this.m_stack.trans().create_task({
                common: {
                    level: cyfs.NDNAPILevel.Router,
                    referer_object: [],
                    flags: 0,
                    dec_id: this.m_decId
                },
                object_id: this.m_fileId.object_id,
                local_path: this.m_savePath,
                auto_start: false,
                device_list: accDeviceList
            });

            if (cr.err) {
                console.error(
                    `create download task(${this.m_fileId.to_base_58()}|${
                        this.m_taskId
                    }) failed, ${cr}.`
                );
                changeFailed(cr.val);
                return cr;
            }

            this.m_taskId = cr.unwrap().task_id;
        }

        const sr = await this.m_stack.trans().start_task({
            common: {
                level: cyfs.NDNAPILevel.Router,
                referer_object: [],
                flags: 0,
                dec_id: this.m_decId
            },
            task_id: this.m_taskId
        });

        if (sr.err) {
            console.error(
                `start download task(${this.m_fileId.to_base_58()}|${this.m_taskId}) failed, ${sr}.`
            );
            changeFailed(sr.val);
            return sr;
        }

        this.m_state = DownloadFileTaskState.START;
        this.emit('state');

        const interval = setInterval(() => {
            if (this.m_state === DownloadFileTaskState.START) {
                this._refresh();
                if (this.m_state !== DownloadFileTaskState.START) {
                    clearInterval(interval);
                }
            }
        }, 1000);
        console.info(`download task(${this.m_fileId.to_base_58()}|${this.m_taskId}) started.`);
        return cyfs.Ok(undefined);
    }

    public async stop(): Promise<cyfs.BuckyResult<void>> {
        console.info(`download task(${this.m_fileId.to_base_58()}|${this.m_taskId}}) will stop.`);
        const r = await this.m_stack.trans().stop_task({
            common: {
                level: cyfs.NDNAPILevel.Router,
                referer_object: [],
                flags: 0,
                dec_id: this.m_decId
            },
            task_id: this.m_taskId!
        });
        if (r.err) {
            console.error(
                `stop download task(${this.m_fileId.to_base_58()}|${this.m_taskId}) failed, ${r}.`
            );
            return r;
        }
        return cyfs.Ok(undefined);
    }
    public async destroy(): Promise<cyfs.BuckyResult<void>> {
        console.info(
            `download task(${this.m_fileId.to_base_58()}|${this.m_taskId}}) will destroy.`
        );

        const r = await this.m_stack.trans().delete_task({
            common: {
                level: cyfs.NDNAPILevel.Router,
                referer_object: [],
                flags: 0,
                dec_id: this.m_decId
            },
            task_id: this.m_taskId!
        });
        if (r.err) {
            console.error(
                `destroy download task(${this.m_fileId.to_base_58()}|${
                    this.m_taskId
                }) failed, ${r}.`
            );
            return r;
        }
        return cyfs.Ok(undefined);
    }
    public async queryFileObject(): Promise<cyfs.BuckyResult<cyfs.File>> {
        if (this.m_file) {
            return cyfs.Ok(this.m_file);
        }

        const gr = await this.m_stack.non_service().get_object({
            common: {
                level: cyfs.NONAPILevel.Router,
                flags: 0,
                target:
                    this.m_mainSource?.object_id ||
                    this.m_owner?.object_id ||
                    this.m_ownerOOD[0]?.object_id
            },
            object_id: this.m_fileId.object_id
        });
        if (gr.err) {
            console.error(
                `get file download task(${this.m_fileId.to_base_58()}|${
                    this.m_taskId
                }) object failed, ${gr}`
            );
            return gr;
        }

        const fileDecoder = new cyfs.FileDecoder();
        const dr = fileDecoder.from_raw(gr.unwrap().object.object_raw);
        if (dr.err) {
            console.error(
                `decode file download task(${this.m_fileId.to_base_58()}|${
                    this.m_taskId
                }) object failed, ${dr}`
            );
            return dr;
        }
        this.m_file = dr.unwrap();
        this.m_total = cyfs.JSBI.toNumber(this.m_file.desc().content().len);

        this.emit('total');
        return cyfs.Ok(this.m_file);
    }

    public async _getOwnerOOD(): Promise<cyfs.BuckyResult<void>> {
        let ownerId: cyfs.ObjectId | undefined;
        if (this.m_owner) {
            ownerId = this.m_owner.object_id;
        } else if (!ownerId) {
            const owner = this.m_file?.desc().owner();
            if (owner?.is_some()) {
                ownerId = owner.unwrap();
            }
        }

        // No owner file, can only rely on the incoming device as the download source
        if (!ownerId) {
            return cyfs.Ok(undefined);
        }

        const rsR = await this.m_stack.util().resolve_ood({
            common: { flags: 0 },
            object_id: ownerId,
            owner_id: ownerId
        });
        if (rsR.err) {
            console.error(
                `download task(${this.m_fileId.to_base_58()}|${
                    this.m_taskId
                }) get ood info failed, ${rsR}.`
            );
            return rsR;
        }

        const { device_list } = rsR.unwrap();
        this.m_ownerOOD = device_list;
        return cyfs.Ok(undefined);
    }

    private async _refresh() {
        let r = await this.m_stack.trans().get_task_state({
            common: {
                level: cyfs.NDNAPILevel.Router,
                referer_object: [],
                flags: 0,
                dec_id: this.m_decId
            },
            task_id: this.m_taskId!
        });
        if (r.err) {
            console.error(
                `refresh download task(${this.m_fileId.to_base_58()}|${
                    this.m_taskId
                }) failed, ${r}.`
            );
            if (r.val.code === cyfs.BuckyErrorCode.NotFound) {
                r = cyfs.Ok({ state: cyfs.TransTaskState.Canceled });
            } else {
                return;
            }
        }
        const state = r.unwrap();

        const oldState = this.m_state;
        switch (state.state) {
            case cyfs.TransTaskState.Downloading:
            case cyfs.TransTaskState.Pending:
                break;
            case cyfs.TransTaskState.Finished:
                this.m_state = DownloadFileTaskState.SUCCESS;
                break;
            case cyfs.TransTaskState.Paused:
                this.m_state = DownloadFileTaskState.STOP;
                break;
            case cyfs.TransTaskState.Canceled:
                this.m_state = DownloadFileTaskState.DELETE;
                break;
            case cyfs.TransTaskState.Err:
                this.m_err = new cyfs.BuckyError(state.error_code!, 'task failed');
                this.m_state = DownloadFileTaskState.FAIL;
                break;
        }

        const speed = state.on_air_state?.download_speed || 0;
        this.m_speed = speed;
        let percent = state.on_air_state?.download_percent || 0;

        if (this.m_state === DownloadFileTaskState.SUCCESS) {
            percent = 100;
        }

        console.info(
            `download task(${this.m_fileId.to_base_58()}|${this.m_taskId}}) state=${
                this.m_state
            }, speed=${speed}, percent=${percent}. r= ${r}`
        );

        if (this.m_percent !== percent) {
            this.m_percent = percent;
            this.emit('progress');
        }
        if (oldState !== this.m_state) {
            this.emit('state');
        }
    }
}
