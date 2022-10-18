import React from 'react';
import { useHistory } from 'react-router-dom';
import { Spin, Tag } from 'antd';
import { toYMDHMS, requestLocal, useRequestShadow } from '../../utils';
import { ResponseMerge, ResponseMergeList } from '../../types/index';
import { useTranslation } from 'react-i18next';
import styles from './Pulls.css';

const Pulls: React.FC = () => {
    const history = useHistory();
    const { data, loading } = useRequestShadow<{ data: ResponseMergeList }>(
        () => requestLocal('merge/list', {}),
        [],
        '获取合并前求 列表失败'
    );
    const { t } = useTranslation();
    if (loading) {
        return <Spin />;
    }
    const resp = data!.data;
    const list = resp.merge_list;

    const toMergeDetail = (item: ResponseMerge) => {
        history.push(`/${item.author_name}/${item.name}/pulls/${item.id}`);
    };

    return (
        <div className={styles.pullMain}>
            <div className={styles.pullHead}>
                <div className={styles.pullContent}>
                    {/* <div className={styles.pullContentItem}>
                        <span>{resp.other}</span>
                        <span>{t('pull.request.dispatch')}</span>
                    </div>
                    <div className={styles.pullContentItem}>
                        <span>{resp.mine}</span>
                        <span>{t('pull.request.create')}</span>
                    </div> */}
                </div>
            </div>
            <div className={styles.pullList}>
                <div className={styles.pullListHead}>
                    <div className={styles.pullListHeadLeft}>
                        {/* <div className={styles.pullStatus}>
                            <span>{resp.open}</span>
                            <span>{t('pull.request.open')}</span>
                        </div>
                        <div className={styles.pullStatus}>
                            <span>{resp.close}</span>
                            <span>{t('pull.request.close')}</span>
                        </div> */}
                    </div>
                </div>
                <div className={styles.pullListMain}>
                    {loading && <Spin />}
                    {list.map((item: ResponseMerge) => {
                        return (
                            <div
                                className={styles.pullListItem}
                                key={item.id}
                                onClick={() => {
                                    toMergeDetail(item);
                                }}
                            >
                                <div className={styles.pullListItemHead}>{item.title}</div>
                                <div className={styles.pullListItemBottom}>
                                    <Tag>{item.user_name}</Tag> opened {toYMDHMS(item.date)}
                                </div>
                            </div>
                        );
                    })}
                </div>
            </div>
        </div>
    );
};

export default Pulls;
