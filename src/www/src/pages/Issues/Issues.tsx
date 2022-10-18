import React from 'react';
import { Spin } from 'antd';
import { toYMDHMS, requestLocal, useRequestShadow } from '../../utils';
import { ResponseIssueItem, ResponseIssueList } from '../../types/index';
import { useTranslation } from 'react-i18next';
import styles from './Issues.module.less';
import { useHistory } from 'react-router-dom';

const Issues: React.FC = () => {
    const history = useHistory();
    const { data, loading } = useRequestShadow<{ data: ResponseIssueList }>(
        () => requestLocal('issue/list', {}),
        [],
        '获取issue 列表失败'
    );
    const { t } = useTranslation();
    if (loading) {
        return <Spin />;
    }

    const resp = data!.data;

    const toDetailPage = (issue: ResponseIssueItem) => {
        history.push(`/${issue.author_name}/${issue.name}/issues/${issue.id}`);
    };

    return (
        <div className={styles.pullMain}>
            <div className={styles.pullHead}>
                {/* <div className={styles.pullContent}>
                    <div className={styles.pullContentItem}>
                        <span>{resp.other}</span>
                        <span>{t('issue.dispatch')}</span>
                    </div>
                    <div className={styles.pullContentItem}>
                        <span>{resp.mine}</span>
                        <span>{t('issue.create')}</span>
                    </div>
                </div> */}
            </div>
            <div className={styles.pullList}>
                <div className={styles.pullListHead}>
                    <div className={styles.pullListHeadLeft}>
                        {/* <div className={styles.pullStatus}>
                            <span>{resp.open}</span>
                            <span>{t('issue.open')}</span>
                        </div>
                        <div className="pull-status">
                            <span>{resp.close}</span>
                            <span>{t('issue.close')}</span>
                        </div> */}
                    </div>
                </div>
                <div className={styles.pullListMain}>
                    {loading && <Spin />}
                    {resp.issues?.map((item: ResponseIssueItem) => {
                        return (
                            <div
                                className={styles.pullListItem}
                                key={item.id}
                                onClick={() => {
                                    toDetailPage(item);
                                }}
                            >
                                <div className={styles.pullListItemHead}>{item.title}</div>
                                <div className={styles.pullListItemBottom}>
                                    opened {toYMDHMS(item.date)}
                                </div>
                            </div>
                        );
                    })}
                </div>
            </div>
        </div>
    );
};

export default Issues;
