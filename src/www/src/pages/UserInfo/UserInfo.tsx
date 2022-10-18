import React, { useState } from 'react';
import { useHistory, useParams } from 'react-router-dom';
import { toYMDHMS, requestLocal } from '../../utils';
import { message } from 'antd';
import { ResponseCheckUser } from '../../types';
import { userInfoAtom } from '../../stores/user';
import UserLogo from '../../components/UserLogo/UserLogo';
import { useRecoilState } from 'recoil';
import { PaginationBottom } from '../../components/Pagination/Pagination';
import { useTranslation } from 'react-i18next';
import UserRepoCheckedImg from '@src/assets/images/user_repo_checked.png';
import UserRepoImg from '@src/assets/images/user_repo.png';
import UserActivityCheckedImg from '@src/assets/images/user_activity_checked.png';
import UserActivityImg from '@src/assets/images/user_activity.png';
import EmailImg from '@src/assets/images/email.png';
import JoinImg from '@src/assets/images/join.png';
import PrivateImg from '@src/assets/images/private.png';
import OpenImg from '@src/assets/images/open.png';
import styles from './UserInfo.module.less';

const UserInfo: React.FC = () => {
    const history = useHistory();
    const { owner, name } = useParams<{ name: string; owner: string }>();
    const [activeKey, setActiveKey] = useState('repo');
    const [repositorys, setRepositorys] = useState<ResponseRepository[]>([]);
    const [count, setCount] = useState(0);
    const [userInfo, setUserInfo] = useState<ServiceResponseUserData>({
        name: '--',
        email: '--',
        userId: owner,
        date: 0
    });
    const [index, setIndex] = useState(0);
    const [user] = useRecoilState(userInfoAtom);
    const pageSize = 10;
    const { t } = useTranslation();

    console.log('match.owner---', owner);
    React.useEffect(() => {
        if (owner === user.id) {
            // 当前用户
            setUserInfo(JSON.parse(JSON.stringify(user)));
            (async function () {
                const r = await requestLocal<{
                    data: ResponseRepository[];
                    count: number;
                }>('repo/list', { page_index: index, page_size: pageSize });
                if (r.err) {
                    console.error('获取仓库列表失败', r);
                    message.error(t('error.user.info.repository.fail'));
                    return;
                }
                console.log('request local', r);
                if (r.data) {
                    setRepositorys(r.data.data);
                    setCount(r.data.count || 0);
                }
            })();
        } else {
            (async function () {
                const r = await requestLocal<ResponseCheckUser>('user/info', {
                    owner_id: owner,
                    name: name
                });
                if (r.err) {
                    console.error('request user/info failed', r.msg);
                    return;
                }
                if (r.data) {
                    setUserInfo(r.data.user!);
                }
                console.log('get UserInfo', r);
            })();
            (async function () {
                const res = await requestLocal<{ data: ResponseRepository[] }>('repo/list', {
                    author_name: name,
                    user_id: owner,
                    page_index: index,
                    page_size: pageSize
                });
                if (res.err) {
                    message.error(t('error.user.info.repository.fail'));
                    return;
                }
                if (res.data) {
                    setRepositorys(
                        res.data.data.filter((ff: ResponseRepository) => {
                            return ff.author_name === name && ff.is_private == 0; // TODO is_private in service
                        })
                    );
                }
            })();
        }
    }, [index, owner]);

    const changeKey = (key: string) => {
        setActiveKey(key);
    };

    const locationRepository = (item: ResponseRepository) => {
        history.push(`/${item.author_name}/${item.name}`);
    };

    const pageSizeChange = (index: number) => {
        setIndex(index);
    };

    console.log('repositorys---', repositorys);
    return (
        <div className={styles.userinfoMain}>
            <div className={styles.userinfoHead}>
                <div className={styles.userinfoTab}>
                    <div
                        className={
                            activeKey == 'repo' ?
                                styles.userinfoTabItemActive :
                                styles.userinfoTabItem
                        }
                        onClick={() => {
                            changeKey('repo');
                        }}
                    >
                        {activeKey == 'repo' ? (
                            <img src={UserRepoCheckedImg} alt="" />
                        ) : (
                            <img src={UserRepoImg} alt="" />
                        )}
                        <span>{t('user.info.repositorys')}</span>
                    </div>
                    <div
                        className={
                            activeKey == 'activity' ?
                                styles.userinfoTabItemActive :
                                styles.userinfoTabItem
                        }
                        onClick={() => {
                            changeKey('activity');
                        }}
                    >
                        {activeKey == 'activity' ? (
                            <img src={UserActivityCheckedImg} alt="" />
                        ) : (
                            <img src={UserActivityImg} alt="" />
                        )}
                        <span>{t('user.info.activity')}</span>
                    </div>
                </div>
            </div>
            <div className={styles.userinfoMainContent}>
                <div className={styles.userinfoMainLeft}>
                    <UserLogo user_id={owner} className={styles.userLogo} />
                    <p className={styles.userinfoName}>{userInfo.name}</p>
                    <div className={styles.personalDetail}>
                        <div className={styles.personalDetailItem}>
                            <img src={EmailImg} alt="" />
                            <span>{userInfo.email}</span>
                        </div>
                        <div className={styles.personalDetailItem}>
                            <img src={JoinImg} alt="" />
                            <span>{`${t('user.info.join')} ${toYMDHMS(userInfo.date)}`}</span>
                        </div>
                    </div>
                </div>
                <div className={styles.userinfoMainRight}>
                    {activeKey === 'repo' ? (
                        <div className={styles.userInfoRepo}>
                            {repositorys.map((repository: ResponseRepository) => (
                                <div className={styles.userInfoRepoItem} key={repository.id}>
                                    <div className={styles.userInfoRepoItemLeft}>
                                        {
                                            <img
                                                src={
                                                    repository.is_private == 1 ?
                                                        PrivateImg :
                                                        OpenImg
                                                }
                                                className={styles.repositoryLogo}
                                                alt=""
                                            />
                                        }
                                    </div>
                                    <div className={styles.userInfoRepoItemRight}>
                                        <div
                                            className={styles.userInfoRepoItemName}
                                            onClick={() => {
                                                locationRepository(repository);
                                            }}
                                        >
                                            {repository.name}
                                        </div>
                                        <div className={styles.userInfoRepoItemTime}>
                                            {`${t('user.info.repository.create')}: ${toYMDHMS(
                                                repository.date!
                                            )}`}
                                        </div>
                                    </div>
                                </div>
                            ))}
                            {repositorys.length > 0 && activeKey === 'repo' && (
                                <PaginationBottom
                                    pageSize={10}
                                    total={count}
                                    onChange={pageSizeChange}
                                />
                            )}
                        </div>
                    ) : (
                        <div></div>
                    )}
                </div>
            </div>
        </div>
    );
};

export default UserInfo;
