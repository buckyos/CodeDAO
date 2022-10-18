import React from 'react';
import { RouteComponentProps, withRouter } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import ActivityLogoIcon from '@src/assets/images/activity_logo.png';
import styles from './RepoActivity.css';

const ReoActivity: React.FC<RouteComponentProps> = ({ history }) => {
    const { t } = useTranslation();

    return (
        <div className={styles.activityContent}>
            <div className={styles.activityContentMain}>
                <div className={styles.activityContentTitle}>{t('home.activity.title')}</div>
                <div className={styles.activityContentInfo}>
                    <div className={styles.text1}>{t('home.activity.prompt')}</div>
                    <div className={styles.text2}>{t('home.activity.desc')}</div>
                    <img className={styles.infoLogo} src={ActivityLogoIcon} />
                    <div
                        className={styles.exploreBtn}
                        onClick={() => {
                            history.push('/explore/organizations');
                        }}
                    >
                        {t('home.activity.explore')}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default withRouter(ReoActivity);
