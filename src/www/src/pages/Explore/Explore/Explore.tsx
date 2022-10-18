import React from 'react';
import { Switch, Route } from 'react-router-dom';
import OrganizationList from '../OrganizationList/OrganizationList';
import ExploreRepositoryList from '../ExploreRepoList/ExploreRepoList';
import UserList from '../UserList/UserList';
import { useTranslation } from 'react-i18next';
import RepoImg from '@src/assets/images/repo.png';
import RepoCheckedImg from '@src/assets/images/repo_checked.png';
import UserLogoImg from '@src/assets/images/user_logo.png';
import UserLogoCheckedImg from '@src/assets/images/user_logo_checked.png';
import OrganizationImg from '@src/assets/images/organization.png';
import OrganizationCheckedImg from '@src/assets/images/organization_checked.png';
import styles from './Explore.module.less';
import { useHistory } from 'react-router-dom';

const Explore: React.FC = () => {
    const history = useHistory();
    const { t } = useTranslation();
    const menu = [
        {
            name: t('explore.repository'),
            route: '/explore/repos',
            src: RepoImg,
            checkedSrc: RepoCheckedImg
        },
        {
            name: t('explore.user'),
            route: '/explore/users',
            src: UserLogoImg,
            checkedSrc: UserLogoCheckedImg
        },
        {
            name: t('explore.organization'),
            route: '/explore/organizations',
            src: OrganizationImg,
            checkedSrc: OrganizationCheckedImg
        }
    ];
    const [activeKey, setActiveKey] = React.useState(0);
    React.useEffect(() => {
        updateActiveKey();
    });

    const updateActiveKey = () => {
        const tail_path = location.hash;
        let activeKey = 0;
        for (const key in menu) {
            const item = menu[key];
            // console.log(item)
            if (tail_path.match(item.route)) {
                activeKey = parseInt(key);
                break;
            }
        }
        setActiveKey(activeKey);
    };

    return (
        <div className={styles.detailContent}>
            <div className={styles.mainInner}>
                <div className={styles.mainLeft}>
                    <h4 className={styles.exploreMenuHeader}>{t('explore.title')}</h4>
                    <div className={styles.exploreMenuWrap}>
                        {menu.map((item, key) => {
                            const className =
                                activeKey == key ? styles.exploreMenuActive : styles.exploreMenu;
                            const imgUrl = activeKey == key ? item.checkedSrc : item.src;
                            return (
                                <div
                                    className={className}
                                    key={item.name}
                                    onClick={() => {
                                        history.push(item.route);
                                        updateActiveKey();
                                    }}
                                >
                                    <div>
                                        {<img src={imgUrl} alt="" />}
                                        <span>{item.name}</span>
                                    </div>
                                </div>
                            );
                        })}
                    </div>
                </div>

                <div className={styles.mainRight}>
                    <Switch>
                        <Route path="/explore/repos">
                            <ExploreRepositoryList />
                        </Route>
                        <Route path="/explore/users">
                            <UserList />
                        </Route>
                        <Route path="/explore/organizations">
                            <OrganizationList />
                        </Route>
                    </Switch>
                </div>
            </div>
        </div>
    );
};

export default Explore;
