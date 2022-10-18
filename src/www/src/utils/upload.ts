import { stack, stackInfo } from './stack';
import { ResponseAddFile } from '../types/index';
import {
    NONAPILevel,
    NONObjectInfo,
    NDNAPILevel,
    TransTaskState,
    DeviceId,
    ObjectId
} from 'cyfs-sdk';
import util from 'util';
const sleep = util.promisify(setTimeout);

export async function addFile(saveFilePath: string): Promise<ResponseAddFile> {
    // 文件上传到cyfs
    const r = await stack.trans().publish_file({
        common: {
            // req_path: '/git',
            dec_id: stackInfo.appID,
            level: NDNAPILevel.Router,
            // target: commonTarget.object_id,
            referer_object: [],
            flags: 0
        },
        owner: stackInfo.owner,
        local_path: saveFilePath,
        chunk_size: 1024 * 1024 * 4 // 这里单位是kB
    });

    if (r.err) {
        console.error(`add file ${saveFilePath} to stack failed, err ${r.val}`);
        return {
            err: r.err,
            msg: 'add file failed'
        };
    }
    const file_id = r.unwrap().file_id;

    // 从noc取回这个FileObject
    const r1 = await stack.non_service().get_object({
        common: {
            level: NONAPILevel.NOC,
            flags: 0
        },
        object_id: file_id
    });
    if (r1.err) {
        return {
            err: r.err,
            msg: 'get object from noc failed'
        };
    }
    const file_resp = r1.unwrap();

    // put object to ood
    await stack.non_service().put_object({
        common: {
            level: NONAPILevel.Router,
            flags: 0,
            target: stackInfo.ood_device_id.object_id
        },
        object: new NONObjectInfo(file_id, file_resp.object.object_raw)
    });
    console.log(`add file success: id ${file_id.toString()}`);

    return {
        err: false,
        msg: '',
        file_id: file_id.toString()
    };
}

interface transactionParamsType {
	targetPath: string;
	fromDeviceID: DeviceId[];
	targetDeviceID: DeviceId;
	fileID: ObjectId;
	commonTarget: DeviceId;
}

interface transactionResponse {
	err: boolean;
	msg: string;
}

// start task  && get task 封装
export async function download(
    params: transactionParamsType
): Promise<transactionResponse> {
    const { targetPath, fromDeviceID, targetDeviceID, fileID, commonTarget } =
		params;

    {
        const r = await stack.non_service().get_object({
            common: {
                dec_id: stackInfo.appID,
                target: commonTarget.object_id,
                flags: 0,
                level: NONAPILevel.Router
            },
            object_id: fileID
        });

        // const [file] = (new FileDecoder()).raw_decode(r.unwrap().object.object_raw).unwrap();
        const put_result = await stack.non_service().put_object({
            common: {
                dec_id: stackInfo.appID,
                target: commonTarget.object_id,
                flags: 0,
                level: NONAPILevel.Router
            },
            object: new NONObjectInfo(fileID, r.unwrap().object.object_raw)
        });
        console.error('get_object:', r);
    }

    const task_common = {
        req_path: '/git',
        dec_id: stackInfo.appID,
        level: NDNAPILevel.Router,
        // target: commonTarget.object_id,
        target: targetDeviceID.object_id, // 目标是runtime
        referer_object: [],
        flags: 0
    };

    const r = await stack.trans().create_task({
        common: task_common,
        object_id: fileID,
        local_path: targetPath,
        device_list: fromDeviceID,
        auto_start: true
    });

    const task_id = r.unwrap().task_id;
    console.error('create task task_id:', task_id);

    if (r.err) {
        // console.log(`add file task for ${targetDeviceID.toString()} failed `, r)
        return {
            err: r.err,
            msg: 'start file task failed'
        };
    }

    let times = 0;
    let finish = false;

    while (times < 80) {
        const resp = (
            await stack.trans().get_task_state({
                common: task_common,
                task_id
            })
        ).unwrap();
        if (resp.state === TransTaskState.Finished) {
            finish = true;
            break;
        }

        times++;
        await sleep(1000);
    }
    if (!finish) {
        return {
            err: true,
            msg: 'time out'
        };
    }

    console.log(`[${fileID.toString()}] get_task_state ok `);
    return {
        err: false,
        msg: ''
    };
}
