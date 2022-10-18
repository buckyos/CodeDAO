import React from 'react';
import { RouteComponentProps, useParams, withRouter } from 'react-router-dom';

import { Button, Tabs, Radio, Input, message, RadioChangeEvent } from 'antd';
import { RightOutlined, DownOutlined } from '@ant-design/icons';
import { requestLocal, requestTarget } from '../../../../utils';
import {
    DiffLineData,
    DiffResult,
    RequestRepositoryMergeCompareFile,
    RequestRepositoryMergeCreate
} from '../../../../common/types';
import CommitDiff from '../../../../components/CommitDiff/CommitDiff';
import style from './RepoPullsCompare.css';
import { useRecoilState } from 'recoil';
import { repositoryCompareInfoAtom } from '../../../../stores/compare';
import { useTranslation } from 'react-i18next';

const { TabPane } = Tabs;

const RepoPullsCompare: React.FC<RouteComponentProps> = ({ history }) => {
    const [mergeType, setMergeType] = React.useState('merge');
    const [title, setTitle] = React.useState('this is merge request title');
    const [buttonLoading, setButtonLoading] = React.useState<boolean>(false);
    const { owner, object_id } = useParams<RepoUrlParams>();
    const [repositoryCompareInfo] = useRecoilState(repositoryCompareInfoAtom);

    const { t } = useTranslation();

    const callback = () => {};

    const createMergeRequest = React.useCallback(async () => {
        setButtonLoading(true);

        // const req: RequestRepositoryMergeCreate = {
        //     repository_owner_id: owner,
        //     id: object_id,
        //     target: repositoryCompareInfo.target,
        //     origin: repositoryCompareInfo.origin,
        //     title: title,
        //     mergeType: mergeType,
        // }
        const r = await requestLocal('repo/merge/create', {
            // repository_owner_id: owner,
            // id: object_id,
            author_name: owner,
            name: object_id,
            target: repositoryCompareInfo.target,
            origin: repositoryCompareInfo.origin,
            title: title,
            merge_type: mergeType
        });
        if (r.err) {
            const msg = '提交失败';
            console.log(msg);
            message.error(msg);
            return;
        }

        message.success(t('success.merge.new'));
        // TODO location
        setTimeout(() => {
            setButtonLoading(false);
            history.push(`/${owner}/${object_id}/pulls`);
        }, 1000);
    }, [title, mergeType]);

    const options = [
        { label: 'rebase', value: 'rebase', disabled: true },
        { label: 'merge', value: 'merge' }
    ];

    return (
        <div>
            <br />
            <br />
            <div>
                <Radio.Group
                    options={options}
                    onChange={(e: RadioChangeEvent) => setMergeType(e.target.value)}
                    value={mergeType}
                    optionType="button"
                />
            </div>
            <br />
            <div>
                <Input value={title} onChange={(event) => setTitle(event.target.value)}></Input>
            </div>
            <Button
                loading={buttonLoading}
                type="primary"
                onClick={() => createMergeRequest()}
                style={{ marginTop: '20px' }}
            >
                {t('repository.pull.create.merge')}
            </Button>

            <Tabs defaultActiveKey="1" onChange={callback}>
                <TabPane tab="Commits" key="1">
                    <div>
                        {repositoryCompareInfo.commits.map((item: CompareCommit, key: number) => {
                            return (
                                <div
                                    key={key}
                                    style={{ display: 'flex', justifyContent: 'space-between' }}
                                >
                                    <span>{item.message}</span>
                                    <span>{item.commit}</span>
                                </div>
                            );
                        })}
                    </div>
                </TabPane>
                <TabPane tab={t('repository.pull.create.file')} key="2">
                    <span style={{ padding: '0 10px' }}>
                        {t('repository.pull.create.file.compare')}
                    </span>
                    <br />

                    {repositoryCompareInfo.diff.map((item: DiffResult, key: number) => {
                        return <RepoPullsCompareFile key={key} data={item}></RepoPullsCompareFile>;
                    })}

                    {/* <CommitDiff diffData={CompareStore.diff } /> */}
                </TabPane>
            </Tabs>
        </div>
    );
};

const RepoPullsCompareFile: React.FC<FileProps> = ({ data }) => {
    const [open, setOpen] = React.useState(false);
    const [content, setContent] = React.useState<DiffLineData[]>([]);
    const [notSupport, setNotSupport] = React.useState(false);
    const [repositoryCompareInfo] = useRecoilState(repositoryCompareInfoAtom);
    const { owner, object_id } = useParams<RepoUrlParams>();

    const onClick = async () => {
        setOpen(!open);
        setNotSupport(false);
        setContent([]);

        console.log('data.fileName', data.fileName);
        const r = await requestLocal<{ data: ResponseCompareFile }>('repo/merge/compare/file', {
            author_name: owner,
            name: object_id,
            target: repositoryCompareInfo.target,
            origin: repositoryCompareInfo.origin,
            file_name: data.file_name
        });
        if (r.err) {
            message.error(`read file err ${r.msg}`);
            return;
        }

        setNotSupport(r.data!.data.notSupport);
        setContent(r.data!.data.content);
    };

    return (
        <div className={style.block} onClick={onClick}>
            <div className={style.header}>
                {open && <DownOutlined className={style.icon} />}
                {!open && <RightOutlined className={style.icon} />}

                <div className={style.diffNumber}>{data.count}</div>
                <div>{data.file_name}</div>
            </div>

            {open && content.length > 0 && notSupport != true && (
                <CommitDiff header={false} diffData={content} />
            )}
            {notSupport && <div>不支持预览此文件</div>}
        </div>
    );
};

export default withRouter(RepoPullsCompare);
