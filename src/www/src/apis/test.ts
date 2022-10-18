/**
 * 测试api
 */

import { checkStack } from '@src/utils/cyfs_helper/stack_wraper';
import * as cyfs from 'cyfs-sdk';

enum Actions {
    CREATE, // 创建
    UPDATE, // 更新
    RETRIEVE, // 查询
    DELETE // 删除
}

interface PubSubObject {
    appName: string;
    decId: string;
    actionType: Actions;
    actionTarget: string;
    describe?: string;
    openURL?: string;
}

const decId = cyfs.ObjectId.from_base_58('9tGpLNnKReYwVv6HMQxtkAxA9N627tLJ4s2d8qa5AyW9').unwrap();

// 通知发布订阅中心，创建词条
export async function notifyCreateAritcle() {
    const stackWraper = checkStack();
    const obj: PubSubObject = {
        appName: 'wiki-creator',
        decId: '9tGpLNnLcSRbbqoYA9zWmBdJ6YyVbQ8S3wUKi9Ru2v71',
        actionType: Actions.CREATE,
        actionTarget: '词条',
        describe: '第一个词条',
        openURL: '没啥描述'
    };
    const textObject = cyfs.TextObject.create(
        cyfs.Some(stackWraper.checkOwner()),
        '',
        'test',
        JSON.stringify(obj)
    );
    const ret = await stackWraper.postObject(textObject, cyfs.TextObjectDecoder, {
        reqPath: 'publish/records',
        decId
    });
    console.log('ret>>>>>>>>>>', JSON.stringify(ret));
}

// 查询people列表
export async function getPeopleList() {
    const stackWraper = checkStack();
    const textObject = cyfs.TextObject.create(cyfs.Some(stackWraper.checkOwner()), '', 'test', '');
    const ret = await stackWraper.postObject(textObject, cyfs.TextObjectDecoder, {
        reqPath: 'pubsub/people_list',
        decId
    });

    const raw = ret.unwrap()!;

    const decoder = new cyfs.TextObjectDecoder();
    const r = decoder.from_raw(raw.encode_to_buf().unwrap());
    if (r.err) {
        const msg = `decode failed, ${r}.`;
        console.error(msg);
        return null;
    }
    const param = r.unwrap().value;

    console.log('ret>>>>>>>>>>', param);
}

// 订阅新人
export async function addPublisher(peopleId = '5r4MYfFSHP9fBMtzQBw3SvRYGztJYaWyTcTitLGWEEjG') {
    const stackWraper = checkStack();
    const obj = {
        peopleId
    };
    const textObject = cyfs.TextObject.create(
        cyfs.Some(stackWraper.checkOwner()),
        '',
        'test',
        JSON.stringify(obj)
    );
    const ret = await stackWraper.postObject(textObject, cyfs.TextObjectDecoder, {
        reqPath: 'subscribe/publishers',
        decId
    });
    const raw = ret.unwrap()!;
    const decoder = new cyfs.TextObjectDecoder();
    const r = decoder.from_raw(raw.encode_to_buf().unwrap());
    if (r.err) {
        const msg = `decode failed, ${r}.`;
        console.error(msg);
        return null;
    }
    const param = r.unwrap().value;
    console.log('ret>>>>>>>>>>', param);
}

// 查询records记录
export async function queryPubSubRecords() {
    const stackWraper = checkStack();
    const obj = {
        startIndex: 0,
        pageSize: 5
    };
    const textObject = cyfs.TextObject.create(
        cyfs.Some(stackWraper.checkOwner()),
        '',
        'test',
        JSON.stringify(obj)
    );
    const ret = await stackWraper.postObject(textObject, cyfs.TextObjectDecoder, {
        reqPath: 'pubsub/objects_record',
        decId
    });
    const raw = ret.unwrap()!;

    const decoder = new cyfs.TextObjectDecoder();
    const r = decoder.from_raw(raw.encode_to_buf().unwrap());
    if (r.err) {
        const msg = `decode failed, ${r}.`;
        console.error(msg);
        return null;
    }
    const param = r.unwrap().value;

    console.log('ret>>>>>>>>>>', param);
}
