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

