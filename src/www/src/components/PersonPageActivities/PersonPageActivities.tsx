import React, { useEffect, useState } from 'react';
import styles from './PersonPageActivities.module.less';
import PersonPageActivitiesItem from '../PersonPageActivitiesItem/PersonPageActivitiesItem';
import { queryPubSubRecords } from '@src/apis/user';
import { PubSubObject } from '@src/types';
import * as cyfs from 'cyfs-sdk';

interface Props {
	listData: PubSubObject[];
}

export default function PersonPageActivities({ listData }: Props) {
    return (
        <div className={styles.box}>
            {listData.map((item) => {
                return (
                    <PersonPageActivitiesItem key={item.createdTimestamp} {...item} />
                );
            })}
        </div>
    );
}
