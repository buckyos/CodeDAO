import * as cyfs from 'cyfs-sdk';
import { DownloadFileTask, DownloadFileTaskState } from './download';

function makeTaskKey(fileId: cyfs.FileId, savePath: string): string {
    return `${fileId.to_base_58()}-${savePath}`;
}

class DownloadTaskMgr {
    private m_stack: cyfs.SharedCyfsStack;
    private m_tasks: Map<string, DownloadFileTask> = new Map();
    public constructor(stack: cyfs.SharedCyfsStack) {
        this.m_stack = stack;
    }

    public createTask(
        fileId: cyfs.FileId,
        savePath: string,
        options: {
            owner?: cyfs.PeopleId;
            mainDeviceIdOrTaskId?: cyfs.DeviceId | string; // 续传任务填TaskId(string)
            accDeviceList: cyfs.DeviceId[];
            decId: cyfs.ObjectId;
        }
    ): { task: DownloadFileTask; isDup: boolean } {
        const key = makeTaskKey(fileId, savePath);
        let task = this.m_tasks.get(key);
        if (!task) {
            task = new DownloadFileTask(fileId, savePath, { ...options, stack: this.m_stack });
            this.m_tasks.set(key, task);

            const handleStateChanged = () => {
                if (task!.state === DownloadFileTaskState.DELETE) {
                    this.m_tasks.delete(key);
                    task!.removeListener('state', handleStateChanged);
                }
            };

            task.on('state', handleStateChanged);
            return { task, isDup: false };
        } else {
            return { task, isDup: true };
        }
    }
}

export let downloadMgr: DownloadTaskMgr | undefined;
export function initDownloadMgr(stack: cyfs.SharedCyfsStack) {
    if (!downloadMgr) {
        downloadMgr = new DownloadTaskMgr(stack);
    }
    return downloadMgr;
}
