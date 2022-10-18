import * as cyfs from 'cyfs-sdk';
import { EnvTarget } from './constant';

export class MetaClientWraper {
    protected m_metaChain: cyfs.MetaClient;
    public constructor(envTarget: EnvTarget) {
        let target;
        switch (envTarget) {
            case EnvTarget.NIGHTLY:
                target = 'dev';
                break;
            case EnvTarget.BETA:
                target = 'test';
                break;
        }
        this.m_metaChain = cyfs.create_meta_client(target);
    }

    public check(): cyfs.MetaClient {
        return this.m_metaChain;
    }
}

let g_metaClient: MetaClientWraper | undefined;

export function init(envTarget: EnvTarget) {
    if (!g_metaClient) {
        g_metaClient = new MetaClientWraper(envTarget);
    }
}

export function checkMetaClient(): MetaClientWraper {
    return g_metaClient!;
}
