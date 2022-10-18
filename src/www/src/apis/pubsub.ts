// 发布订阅中心相关api
import { checkStack } from '@src/utils/cyfs_helper/stack_wraper';
import * as cyfs from 'cyfs-sdk';
import { ActionTarget } from '@src/types';
import {
    DEC_ID_BASE_58,
    DEC_ID,
    APP_NAME,
    APP_OPEN_URL
} from '@src/constants';
import { PubSubObject } from '@src/utils/protos/pubSubObject';

enum Actions {
	CREATE, // 创建
	UPDATE, // 更新
	RETRIEVE, // 查询
	DELETE, // 删除
}

const PUBSUB_SEC_ID = '9tGpLNnGNRjsiwPkKeC9gj49rpDAb44YMe3daW3yiwgj';

const decId = cyfs.ObjectId.from_base_58(PUBSUB_SEC_ID).unwrap();

// 通知发布订阅中心，创建仓库
export async function notifyCreateRepository(describe = '') {
    const stackWraper = checkStack();
    const obj = {
        appName: APP_NAME,
        decId: DEC_ID_BASE_58,
        actionType: Actions.CREATE,
        actionTarget: ActionTarget.REPOSITORY,
        describe,
        openURL: APP_OPEN_URL,
        owner: stackWraper.checkOwner()
    };
    const pubsubObj = PubSubObject.create(obj);
    const ret = await stackWraper.postObject(pubsubObj, cyfs.TextObjectDecoder, {
        reqPath: 'publish/records',
        decId
    });
    console.log('notify pubsub center result:', JSON.stringify(ret));
}
