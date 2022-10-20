import React from 'react';
import styles from './PersonPageCommonTitle.module.less';
import defaultIcon from '@src/assets/images/logo.png';

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
