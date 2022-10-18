import React, { useState } from 'react';
import { Spin } from 'antd';
import { responseUser } from '@src/types';
import { getPageIndex, requestLocal, useRequest } from '../../../utils/index';
import { useHistory } from 'react-router-dom';
import UserLogo from '../../../components/UserLogo/UserLogo';
import { PaginationBottom } from '../../../components/Pagination/Pagination';
import { useTranslation } from 'react-i18next';
import styles from './UserList.css';

const UserList: React.FC = () => {
    const [userName, setUserName] = useState('');
    const [search, setSearch] = useState(false);
    const pageSize = 10;
    const index = getPageIndex();
    const { data, loading, count } = useRequest(
        () =>
            requestLocal('user/list', {
                user_name: userName,
                page_index: index,
                page_size: pageSize
            }),
        [index, search],
        '获取用户列表失败'
    );
    const history = useHistory();
    const { t } = useTranslation();

    const searchUser = () => {
        setSearch(!search);
        history.push('/explore/users');
    };

    console.log(data);

    const pageSizeChange = (index: number) => {
        history.push(`/explore/users?pageIndex=${index}`);
    };

    return (
        <div className={styles.mainRight}>
            <div className={styles.searchInput}>
                <input
                    type="text"
                    placeholder={t('explore.serach.input')}
                    onChange={(e) => {
                        setUserName(e.target.value);
                    }}
                />
                <span
                    onClick={() => {
                        searchUser();
                    }}
                >
                    {t('explore.serach')}
                </span>
            </div>
            <div style={{ marginTop: 20 }}>
                {loading && <Spin />}
                <div className={styles.exploreList}>
                    {data.map((user: responseUser, i: number) => (
                        <div className={styles.exploreListItem} key={user.id}>
                            <div className={styles.exploreListItemLeft}>
                                <UserLogo user_id={user.owner_id} />
                            </div>
                            <div className={styles.exploreListItemRight}>
                                <div
                                    className={styles.exploreListItemName}
                                    onClick={() => {
                                        history.push(`/userInfo/${user.name}/${user.id}`);
                                    }}
                                >
                                    {user.name}
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
                {false && data.length > 0 && (
                    <PaginationBottom pageSize={10} total={count} onChange={pageSizeChange} />
                )}
            </div>
        </div>
    );
};

export default UserList;
