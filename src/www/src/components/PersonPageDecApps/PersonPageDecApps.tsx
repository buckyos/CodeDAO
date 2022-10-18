import React, { useEffect, useState } from 'react';
import styles from './PersonPageDecApps.module.less';
import Icon from '@src/assets/images/personal_page/dec_app.png';
import PersonPageCommonTitle from '../PersonPageCommonTitle/PersonPageCommonTitle';

interface Props {
	title?: string;
	icon?: string;
}

export default function PersonPageDecApps() {
    return (
        <div className={styles.box}>
            <PersonPageCommonTitle icon={Icon} title="DEC Apps" />
        </div>
    );
}
