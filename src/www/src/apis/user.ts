import * as cyfs from 'cyfs-sdk';
import { checkStack } from '../utils/cyfs_helper/stack_wraper';
import { checkMetaClient } from '../utils/cyfs_helper/meta_client';
import {
    PeopleInfo,
    postQueryPubSubPeopleListRouterResponseParam,
    PubSubObject,
    commonResponse
} from '../types';
import { fileIdToCyfsURL, parseTextObject } from '../utils';
import { PUBSUB_CENTER_DEC_ID } from '../constants';

/**
 * 查询people信息
 * @param peopleId
 * @returns
 */
export async function queryPeopleInfo(peopleId: cyfs.PeopleId): Promise<PeopleInfo | null> {
    const stack = checkStack();
    const gr = await stack.getObject(peopleId.object_id, cyfs.PeopleDecoder, {
        level: cyfs.NONAPILevel.NOC
    });

    if (gr.err) {
        console.error(`get people object(${peopleId.object_id.to_base_58()}) failed, ${gr}.`);
        return null;
    }

    const metaClient = checkMetaClient();
    const r = await metaClient.check().getDesc(peopleId.object_id);
    if (r.err) {
        console.error(`find poeple(${peopleId.object_id.to_base_58()}) failed, ${r}.`);
        return null;
    }
    const meta = r.unwrap();
    const people = meta.match({ People: (people) => people });
    if (!people) {
        const msg = `find people(${peopleId.object_id.to_base_58()}) failed, not people?`;
        console.error(msg);
        return null;
    } else {
        const name = people.name();
        const fileIdObj = people.icon();
        const fileId = fileIdObj ? fileIdObj.to_base_58() : '';
        const iconURL = fileIdToCyfsURL(peopleId.to_base_58(), fileId);
        const obj: PeopleInfo = {
            name: name ? name : peopleId.object_id.to_base_58(),
            iconURL
        };
        return obj;
    }
}

// 查询people列表
export async function getPubSubPeopleList(peopleId: string) {
    const stackWraper = checkStack();
    const textObject = cyfs.TextObject.create(cyfs.Some(stackWraper.checkOwner()), '', 'test', '');
    const target = cyfs.PeopleId.from_base_58(peopleId).unwrap().object_id;
    const ret = await stackWraper.postObject(textObject, cyfs.TextObjectDecoder, {
        reqPath: 'pubsub/people_list',
        decId: PUBSUB_CENTER_DEC_ID,
        target
    });
    const jsonStr = parseTextObject(ret.unwrap()!);
    const parseObj = JSON.parse(jsonStr!) as commonResponse;
    const param = parseObj.msg as postQueryPubSubPeopleListRouterResponseParam;
    console.log('getPubSubPeopleList-------------------:', jsonStr);
    return param;
}

// 查询records记录
export async function queryPubSubRecords(
    target: cyfs.ObjectId,
    startIndex = 0,
    pageSize = 5
): Promise<PubSubObject[] | null> {
    const stackWraper = checkStack();
    const obj = {
        startIndex,
        pageSize
    };
    const textObject = cyfs.TextObject.create(
        cyfs.Some(stackWraper.checkOwner()),
        'query-pubsub-records',
        'query-pubsub-records',
        JSON.stringify(obj)
    );
    const ret = await stackWraper.postObject(textObject, cyfs.TextObjectDecoder, {
        reqPath: 'pubsub/objects_record',
        decId: PUBSUB_CENTER_DEC_ID,
        target
    });
    const jsonStr = parseTextObject(ret.unwrap()!);
    const parseObj = JSON.parse(jsonStr!) as commonResponse;
    const param = parseObj.msg as PubSubObject[];
    console.log('ret----------:', jsonStr);
    return param;
}

// 订阅新人
export async function subscribePublisher(peopleId = '') {
    if (!peopleId) {
        const msg = 'peopleId invalid.';
        console.error(msg);
        const obj: commonResponse = {
            err: 1,
            msg
        };
        return obj;
    }
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
        decId: PUBSUB_CENTER_DEC_ID
    });
    const jsonStr = parseTextObject(ret.unwrap()!);
    const parseObj = JSON.parse(jsonStr!) as commonResponse;
    return parseObj;
}
