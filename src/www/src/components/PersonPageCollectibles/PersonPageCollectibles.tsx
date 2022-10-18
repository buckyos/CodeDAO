import React, { useEffect, useState } from 'react';
import styles from './PersonPageCollectibles.module.less';
import Icon from '@src/assets/images/personal_page/collectibles.png';
import PersonPageCommonTitle from '../PersonPageCommonTitle/PersonPageCommonTitle';

interface Props {
	title?: string;
	icon?: string;
}

export default function PersonPageCollectibles() {
    return (
        <div className={styles.box}>
            <PersonPageCommonTitle icon={Icon} title="Collectibles" />
        </div>
    );
}
