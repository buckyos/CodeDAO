export * from './rpc_def';
export * from './text';
export enum AppObjectType {
    PUBSUB_OBJECT = 32768
}
export interface PeopleInfo {
    name: string;
    iconURL?: string;
}

export const ActionsArrayZh = ['新建了', '更新了', '查询了', '删除了'];
export const ActionsArrayEn = ['Created', 'Updated', 'Retrieved', 'Deleted'];

export enum Actions {
    CREATE, // 创建
    UPDATE, // 更新
    RETRIEVE, // 查询
    DELETE // 删除
}
export interface PubSubObject {
    appName: string;
    decId: string;
    actionType: Actions;
    actionTarget: string;
    describe?: string;
    openURL?: string;
    ownerId?: string;
    createdTimestamp?: number;
}

export interface postQueryPubSubPeopleListRouterResponseParam {
    subscribers: string[]; // 订阅人列表
    publishers: string[]; // 订阅的发布人列表
}

export interface commonResponse {
    err: number;
    msg: unknown;
}

export enum ActionTarget {
    REPOSITORY = 'Repository'
}
