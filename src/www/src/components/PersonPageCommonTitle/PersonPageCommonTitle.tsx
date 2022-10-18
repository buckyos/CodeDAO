import React, { useEffect, useState } from 'react';
import styles from './PersonPageCommonTitle.module.less';
import defaultIcon from '@src/assets/images/logo.png';
import * as cyfs from 'cyfs-sdk';
import { queryPeopleInfo } from '../../apis/user';

interface Props {
	title?: string;
	icon?: string;
}

export default function PersonPageCommonTitle({ title = '', icon }: Props) {
    return (
        <div className={styles.box}>
            <img className={styles.icon} src={icon ? icon : defaultIcon} />
            <div className={styles.title}>{title}</div>
        </div>
    );
}
