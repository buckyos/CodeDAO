import React from 'react';
import styles from './PersonPageActivities.module.less';
import PersonPageActivitiesItem from '../PersonPageActivitiesItem/PersonPageActivitiesItem';
import { PubSubObject } from '@src/types';

interface Props {
    listData: PubSubObject[];
}

export default function PersonPageActivities({ listData }: Props) {
    return (
        <div className={styles.box}>
            {listData.map((item) => {
                return <PersonPageActivitiesItem key={item.createdTimestamp} {...item} />;
            })}
        </div>
    );
}
