import React from 'react';
import { withRouter, Link, RouteComponentProps } from 'react-router-dom';
import { Table, Spin } from 'antd';
import { requestLocal, requestTarget, useRequest } from '@src/utils/index';
import { useTranslation } from 'react-i18next';
import styles from './RepoPulls.module.less';

const RepoPulls: React.FC<RouteComponentProps> = ({ history, match }) => {
    const { owner, object_id } = match.params as RepoUrlParams;
    const { t } = useTranslation();
    const columns = [
        {
            title: t('repository.pull.list.title'),
            dataIndex: 'title',
            /* eslint-disable */
            render: (text: string, row: any) => {
                const url = `/${owner}/${object_id}/pulls/${row.id}`;
                return <Link to={url}>{text}</Link>;
            }
        },
        {
            title: t('repository.pull.list.status'),
            dataIndex: 'status'
        }
    ];

    const { data, loading } = useRequest(
        () =>
            requestLocal('repo/merges', {
                // owner: owner,
                // id: object_id,
                author_name: owner,
                name: object_id
            }),
        [],
        'fetch merge list data failed'
    );

    if (loading) {
        return <Spin />;
    }

    const locationCreate = () => {
        history.push(`/${owner}/${object_id}/pulls/create`);
    };

    return (
        <div className={styles.repoMerage}>
            <div>
                <button className={styles.repoPullBtn} onClick={locationCreate}>
                    {t('repository.pull.create.btn')}
                </button>
            </div>

            <div>
                <Table columns={columns} dataSource={data} rowKey="id" />
            </div>
        </div>
    );
};

export default withRouter(RepoPulls);
