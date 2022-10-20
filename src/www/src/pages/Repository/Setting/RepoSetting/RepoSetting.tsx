import React from 'react';
import { Switch, useHistory, useParams, Route, useLocation } from 'react-router-dom';
import { classSet } from '@src/utils/index';
import RepoSettingMember from '../Member/Member';
import RepoSettingDefault from '../Default/Default';
import { useRecoilState } from 'recoil';
import { userInfoAtom } from '@src/stores/user';
import { useTranslation } from 'react-i18next';
import styles from './RepoSetting.css';

const RepoSetting: React.FC = () => {
    const { object_id, owner } = useParams<RepoUrlParams>();
    const history = useHistory();
    const locationState = useLocation();
    const [index, setActiveIndex] = React.useState(0);
    const [userInfo] = useRecoilState(userInfoAtom);
    const isAdmin = userInfo.owner === owner;
    const { t } = useTranslation();

    const location = React.useCallback((subPath: string) => {
        return () => {
            history.push(`/${owner}/${object_id}/setting${subPath}`);
        };
    }, []);

    const baseMap = [
        { name: 'Repository', navPath: '/' },
        { name: t('repository.settings.tabbar.member'), navPath: '/access' },
        { name: 'Hooks', navPath: '/hooks' },
        { name: 'Branch', navPath: '/branch' }
    ];

    const settingNavMap = isAdmin ? [{ name: t('repository.settings.tabbar.repository'), navPath: '' }, ...baseMap] : baseMap;

    React.useEffect(() => {
        const current = locationState.pathname.replace(`/${owner}/${object_id}/setting`, '');
        const active = settingNavMap.findIndex((item) => {
            return item.navPath === current;
        });

        if (active == -1) {
            setActiveIndex(0);
        } else {
            setActiveIndex(active);
        }
    }, [locationState.pathname]);

    return (
        <div className={styles.main}>
            <div className={styles.left}>
                {settingNavMap.map((item, key) => {
                    const cls = classSet([styles.nav, { [styles.active]: key === index }]);
                    return (
                        <div key={key} className={cls} onClick={location(item.navPath)}>
                            {item.name}
                        </div>
                    );
                })}
            </div>
            <div className={styles.right}>
                <Switch>
                    <Route exact path="/:owner/:object_id/setting">
                        <RepoSettingDefault />
                    </Route>
                    <Route exact path="/:owner/:object_id/setting/access">
                        <RepoSettingMember />
                    </Route>
                    <Route exact path="/:owner/:object_id/setting/hooks">
                        <div className={styles.title}>
                            <h1>Hooks</h1>
                        </div>
                        <div>{t('repository.developing')}</div>
                    </Route>
                    <Route exact path="/:owner/:object_id/setting/branch">
                        <div className={styles.title}>
                            <h1>branch</h1>
                        </div>
                        <div>{t('repository.developing')}</div>
                    </Route>
                </Switch>
            </div>
        </div>
    );
};

export default RepoSetting;
