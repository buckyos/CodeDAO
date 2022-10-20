import React, { useEffect, useState } from 'react';
import styles from './UserCommonInfo.module.less';
import defaultIcon from '@src/assets/images/default_head.png';
import * as cyfs from 'cyfs-sdk';
import { queryPeopleInfo } from '@src/apis/user';

interface Props {
    peopleId: string | cyfs.PeopleId;
    isShowId?: boolean;
    introStr?: string;
    iconWidth?: string;
    iconHeight?: string;
}

export default function UserCommonInfo({
    peopleId,
    introStr,
    isShowId = false,
    iconWidth = '100px',
    iconHeight = '100px'
}: Props) {
    const peopleIdStr = typeof peopleId === 'string' ? peopleId : peopleId.to_base_58();
    const [name, setName] = useState('');
    const [iconURL, setIconURL] = useState('');
    const queryInfo = async () => {
        if (typeof peopleId === 'string') peopleId = cyfs.PeopleId.from_base_58(peopleId).unwrap();
        const ret = await queryPeopleInfo(peopleId);
        if (ret) {
            setName(ret.name);
            setIconURL(ret.iconURL ? ret.iconURL : '');
        }
    };
    useEffect(() => {
        queryInfo();
    }, [peopleId]);
    return (
        <div className={styles.box}>
            <img
                className={styles.icon}
                style={{ width: iconWidth, height: iconHeight }}
                src={iconURL ? iconURL : defaultIcon}
                data-people-id={peopleIdStr}
            />
            <div className={styles.infoBox}>
                <div className={styles.name}>
                    {name}
                    {isShowId ? `(${peopleId})` : ''}
                </div>
                <div className={styles.others}>{introStr}</div>
            </div>
        </div>
    );
}
