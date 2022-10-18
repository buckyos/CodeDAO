import React, { useEffect, useState } from 'react';
import { Pagination, Spin } from 'antd';
import { getPageIndex, requestLocal, toYMDHMS, useRequest } from '../../../utils/index';
import { useHistory } from 'react-router-dom';
import OrganizationLogo from '../../../components/OrganizationLogo/OrganizationLogo';
import { PaginationBottom } from '../../../components/Pagination/Pagination';
import { useTranslation } from 'react-i18next';
import styles from './OrganizationList.css';

const OrganizationList: React.FC = () => {
    const [organizationName, setOrganizationName] = React.useState('');
    const [search, setSearch] = React.useState(false);
    const history = useHistory();
    const pageSize = 10;
    const index = getPageIndex();
    const { data, loading, count } = useRequest(
        () =>
            requestLocal('organization/list', {
                organization_name: organizationName,
                page_index: index,
                page_size: pageSize
            }),
        [index, search],
        '获取group列表失败'
    );
    const { t } = useTranslation();

    const searchOrganization = () => {
        setSearch(!search);
        history.push('/explore/organizations');
    };

    const pageSizeChange = (index: number) => {
        history.push(`/explore/organizations?pageIndex=${index}`);
    };

    const gotoOrganization = React.useCallback((item: ResponseOrganizationList) => {
        return () => {
            history.push(`/organization/${item.name}`);
        };
    }, []);

    return (
        <div className={styles.mainRight}>
            <div className={styles.searchInput}>
                <input
                    type="text"
                    placeholder={t('explore.serach.input')}
                    onChange={(e) => {
                        setOrganizationName(e.target.value);
                    }}
                />
                <span
                    onClick={() => {
                        searchOrganization();
                    }}
                >
                    {t('explore.serach')}
                </span>
            </div>
            <div style={{ marginTop: 20 }}>
                {loading && <Spin />}
                <div className={styles.exploreList}>
                    {data.map((org: ResponseOrganizationList) => (
                        <div
                            onClick={gotoOrganization(org)}
                            className={styles.exploreListItem}
                            key={org.id}
                        >
                            <div className={styles.exploreListItemLeft}>
                                <OrganizationLogo user_id={org.id} />
                            </div>
                            <div className={styles.exploreListItemRight}>
                                <div className={styles.exploreListItemName}>{org.name}</div>
                                <div className={styles.exploreListItemTime}>
                                    {`${t('explore.organization.create')}： ${toYMDHMS(org.date)}`}
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

export default OrganizationList;
