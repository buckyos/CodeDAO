import React from 'react';
import styles from './PersonPageActivitiesItem.module.less';
import Icon from '@src/assets/images/personal_page/app_default_logo.png';
import UserCommonInfo from '@src/components/UserCommonInfo/UserCommonInfo';
import { PubSubObject, Actions, ActionsArrayEn } from '@src/types';
import { getLocalTime } from '@src/utils';

interface Props extends Partial<PubSubObject> {
    icon?: string;
}

export default function PersonPageActivitiesItem({
    appName = 'test app',
    decId,
    actionType = Actions.CREATE,
    actionTarget = 'test object',
    describe = '',
    openURL,
    ownerId = '5r4MYfFbqqyqoA4RipKdGEKQ6ZSX3JzNRaEpMPKiKWAQ',
    createdTimestamp = 1658912707689,
    icon
}: Props) {
    const actionTxt = ActionsArrayEn[actionType];
    const displayTime = getLocalTime(createdTimestamp);
    return (
        <div className={styles.box}>
            <div className={styles.headBox}>
                <div className={styles.headTxt}>
                    {displayTime} · from {appName}
                </div>
                <img className={styles.icon} src={icon ? icon : Icon} />
            </div>

            <div className={styles.content}>
                <UserCommonInfo
                    peopleId={ownerId}
                    introStr="A brief intro"
                    isShowId
                    iconHeight="40px"
                    iconWidth="40px"
                />
                <div className={styles.descBox}>
                    <h3 style={{ fontStyle: 'italic' }}>{actionTxt}</h3>&nbsp;&nbsp;
                    <h3>{actionTarget}</h3>&nbsp;&nbsp;
                    <h3 style={{ color: 'red' }}>{describe}</h3>
                </div>
            </div>
        </div>
    );
}
