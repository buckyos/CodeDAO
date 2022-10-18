import React from 'react';
import { withRouter, RouteComponentProps } from 'react-router-dom';
import { Button, message, Spin, Tag } from 'antd';
import { RequestRepositoryMergeDetail, ResponseMerge } from '../../../../types';
// import {RepoMergeUrlParams} from "../../../../@types"
import { requestLocal, requestTarget, useRequestShadow } from '../../../../utils/index';
import { useTranslation } from 'react-i18next';
import styles from './RepoPullsDetail.css';

const RepoPullsDetail: React.FC<RouteComponentProps> = ({ match }) => {
    const [buttonLoading, setButtonLoading] = React.useState(false);
    const { owner, object_id, pull_id } = match.params as RepoMergeUrlParams;
    const { t } = useTranslation();

    const { data, loading, request } = useRequestShadow<{ data: ResponseMerge }>(
        () =>
            requestLocal('repo/merge/detail', {
                // owner: owner,
                // id: object_id,
                author_name: owner,
                name: object_id,
                merge_id: pull_id
            }),
        [pull_id],
        'fetch merge data failed'
    );

    if (loading) {
        return <Spin />;
    }

    const mr = data!.data;

    console.log('data', data);

    const acceptMerge = async () => {
        setButtonLoading(true);
        const req: RequestRepositoryMergeDetail = {
            author_name: owner,
            name: object_id,
            merge_id: pull_id
        };
        const r = await requestLocal('repo/merge/accept', req);
        if (r.err) {
            console.log(' merge repo failed');
            message.error(r.msg);
            return;
        }
        message.success(t('success.merge.detail.status'));
        setButtonLoading(false);
        request();
    };

    return (
        <div className={styles.repoPullDetail}>
            <div>
                <h1>
                    {t('repository.pull.detail.title')}: {mr.title}
                </h1>

                <div className={styles.repoPullStyle}>
                    <span>{t('repository.pull.detail.branch')}</span>
                    <Tag style={{ margin: '0 5px' }}>{mr.origin_branch}</Tag>
                    <span>{t('repository.pull.detail.pull')}</span>
                    <Tag style={{ margin: '0 5px' }}>{mr.target_branch}</Tag>
                </div>

                <div className={styles.repoPullStyle}>
                    {t('repository.pull.detail.method')}: {mr.merge_type}
                </div>
            </div>
            {mr.status == 'open' && (
                <Button type="primary" loading={buttonLoading} onClick={() => acceptMerge()}>
                    {buttonLoading ?
                        t('repository.pull.detail.status.processing') :
                        t('repository.pull.detail.status.accept')}
                </Button>
            )}
            {mr.status == 'close' && <Tag>{t('repository.pull.detail.status.finish')}</Tag>}
        </div>
    );
};

export default withRouter(RepoPullsDetail);
