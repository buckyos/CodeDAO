import { EventEmitter } from 'events';
import * as cyfs from 'cyfs-sdk';

export class PublishFileTask extends EventEmitter {
    private m_filePath: string;
    private m_stack: cyfs.SharedCyfsStack;
    private m_chunkSize: number;
    private m_decId: cyfs.ObjectId;
    public constructor(
        filePath: string,
        options: { stack: cyfs.SharedCyfsStack; chunkSize?: number; decId: cyfs.ObjectId }
    ) {
        super();
        this.m_filePath = filePath;
        this.m_stack = options.stack;
        this.m_chunkSize = options.chunkSize || 4 << 20;
        this.m_decId = options.decId;
    }

    public async publish(): Promise<cyfs.BuckyResult<cyfs.FileId>> {
        const r = await this.m_stack.trans().publish_file({
            common: {
                level: cyfs.NDNAPILevel.Router,
                flags: 0,
                referer_object: [],
                dec_id: this.m_decId
            },
            owner: this.m_stack.local_device().desc().owner()!.unwrap(),
            local_path: this.m_filePath,
            chunk_size: this.m_chunkSize
        });
        return r.map((v) => cyfs.FileId.try_from_object_id(v.file_id).unwrap());
    }
}
