import React from 'react';
import { useHistory } from 'react-router-dom';
import { Spin } from 'antd';
import { getPageIndex, requestLocal, toYMDHMS, useRequest } from '../../../utils';
import { PaginationBottom } from '../../../components/Pagination/Pagination';
import { useTranslation } from 'react-i18next';
import PrivateImg from '@src/assets/images/private.png';
import OpenImg from '@src/assets/images/open.png';
import styles from './ExploreRepoList.css';

const ExploreRepositoryList: React.FC = () => {
    const [repoName, setRepoName] = React.useState('');
    const [search, setSearch] = React.useState(false);
    const history = useHistory();
    const pageSize = 10;
    const index = getPageIndex();
    const { data, loading, count } = useRequest(
        () =>
            requestLocal('repo/global/list', {
                repo_name: repoName,
                page_index: index,
                page_size: pageSize
            }),
        [index, search],
        '获取仓库列表失败'
    );
    const { t } = useTranslation();

    const locationRepository = (item: ResponseRepository) => {
        history.push(`/${item.author_name}/${item.name}`);
    };

    const searchRepository = () => {
        setSearch(!search);
        history.push('/explore/repos');
    };

    const pageSizeChange = (index: number) => {
        history.push(`/explore/repos?pageIndex=${index}`);
    };

    return (
        <div className={styles.mainRight}>
            <div className={styles.searchInput}>
                <input
                    type="text"
                    placeholder={t('explore.serach.input')}
                    onChange={(e) => {
                        setRepoName(e.target.value);
                    }}
                />
                <span
                    onClick={() => {
                        searchRepository();
                    }}
                >
                    {t('explore.serach')}
                </span>
            </div>
            <div style={{ marginTop: 20 }}>
                {loading && <Spin />}
                <div>
                    {data.map((repository: ResponseRepository) => (
                        <div className={styles.exploreListItem} key={repository.id}>
                            <div className={styles.exploreListItemLeft}>
                                {
                                    <img
                                        src={repository.is_private == 1 ? PrivateImg : OpenImg}
                                        className={styles.repositoryLogo}
                                        alt=""
                                    />
                                }
                            </div>
                            <div className={styles.exploreListItemRight}>
                                <div
                                    className={styles.exploreListItemName}
                                    onClick={() => {
                                        locationRepository(repository);
                                    }}
                                >
                                    {repository.author_name}/
                                    {repository.name}
                                </div>
                                <div className={styles.exploreListItemTime}>
                                    {`${t('explore.repository.create')}： ${toYMDHMS(
                                        repository.date!
                                    )}    `}
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

export default ExploreRepositoryList;
