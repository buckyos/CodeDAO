import React from 'react';
import { useHistory } from 'react-router-dom';
import HeaderLogin from '../HeaderLogin/HeaderLogin';
import { useTranslation } from 'react-i18next';
import LogoIcon from '@src/assets/images/logo.png';
import styles from './Header.css';

const Header: React.FC = () => {
    const history = useHistory();
    const { t } = useTranslation();

    const linkHome = () => {
        history.push('/');
    };

    return (
        <div className={styles.header}>
            <div className={styles.headerInner}>
                <div className={styles.headerLeftWrap}>
                    <div className={styles.headerLeft} onClick={() => linkHome()}>
                        <img className={styles.cyfsGitLogo} src={LogoIcon} />
                        <span className={styles.cyfsGitName}>CodeDAO</span>
                    </div>
                    <div
                        className={styles.headerLeftContent}
                        onClick={() => {
                            history.push('/explore/organizations');
                        }}
                    >
                        <span> {t('header.explore')} </span>
                    </div>
                    <div
                        className={styles.headerLeftContent}
                        onClick={() => {
                            history.push('/pulls');
                        }}
                    >
                        <span> {t('header.pull.request')} </span>
                    </div>
                    <div
                        className={styles.headerLeftContent}
                        onClick={() => {
                            history.push('/issue');
                        }}
                    >
                        <span> {t('header.issue')} </span>
                    </div>

                    <div
                        className={styles.headerLeftContent}
                        onClick={() => {
                            history.push('/dao');
                        }}
                    >
                        <span> DAO </span>
                    </div>
                    {/* 
		    <div
                        className={styles.headerLeftContent}
                        onClick={() => {
                            history.push('/personal_page/self');
                        }}
                    >
			<span> {t('header.personal_page')} </span> 
                    </div>
			*/}
                </div>
                <HeaderLogin />
            </div>
        </div>
    );
};

export default Header;
