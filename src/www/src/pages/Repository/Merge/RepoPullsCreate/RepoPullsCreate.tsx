import React, { useEffect } from 'react';
import { RouteComponentProps, withRouter } from 'react-router-dom';
import { Button, message, Select } from 'antd';
import { requestLocal, requestTarget } from '@src/utils/index';
import RepoPullsCompare from '../RepoPullsCompare/RepoPullsCompare';
import _ from 'lodash';
import { branchesAtom } from '@src/stores/repository';
import { useRecoilState } from 'recoil';
import { repositoryCompareInfoAtom, useCleanCompareInfo } from '@src/stores/compare';
import { useTranslation } from 'react-i18next';
import styles from './RepoPullsCreate.css';
const { Option } = Select;

const RepoPullsCreate: React.FC<RouteComponentProps> = ({ match }) => {
    const [diffResult, setDiffResult] = React.useState(0);
    const { owner, object_id } = match.params as RepoUrlParams;
    const [branches] = useRecoilState(branchesAtom);
    const [repositoryCompareInfo, setRepositoryCompareInfo] =
        useRecoilState(repositoryCompareInfoAtom);
    const cleanCompareInfo = useCleanCompareInfo();
    const { t } = useTranslation();
    const compareInfo: CompareBranch = {
        origin: repositoryCompareInfo.origin,
        target: repositoryCompareInfo.target
    };

    useEffect(() => {
        return () => {
            cleanCompareInfo();
        };
    }, []);

    /* eslint-disable */
    const onChange = async (value: any, key: 'origin' | 'target') => {
        compareInfo[key] = value;
        if (_.isEmpty(compareInfo.target) || _.isEmpty(compareInfo.origin)) {
            setRepositoryCompareInfo({
                commits: [],
                diff: [],
                target: compareInfo.target,
                origin: compareInfo.origin
            });
            return;
        }

        console.log('trigger diff');
        if (compareInfo.target == compareInfo.origin) {
            setRepositoryCompareInfo({
                ...repositoryCompareInfo,
                ...compareInfo
            });
            setDiffResult(1);
            return;
        }
        const r = await requestLocal<{ data: ResponseMergeCompare }>('repo/merge/compare', {
            // owner: owner,
            // id: object_id,
            target: compareInfo.target,
            origin: compareInfo.origin,
            author_name: owner,
            name: object_id
        });
        if (r.err) {
            message.error(`fetch compare err ${r.msg}`);
            return;
        }
        const data = r.data!.data;
        setRepositoryCompareInfo({
            ...compareInfo,
            commits: data.commits,
            diff: data.diff
        });

        console.log('diff resp', data.diff);
        setDiffResult(2);
    };

    return (
        <div>
            <div className={styles.repoPullCreat}>
                <h1 className={styles.repoPullCreatTitle}>{t('repository.pull.create.title')}</h1>
                <div className={styles.repoPullCreatDesc}>
                    {t('repository.pull.create.description')}
                </div>

                <div className={styles.repoPullMain}>
                    <span className={styles.repoPullSpan}>
                        {t('repository.pull.create.source.branch')}：
                    </span>
                    <Select
                        className={styles.repoPullSelect}
                        allowClear
                        onChange={(value) => onChange(value, 'origin')}
                    >
                        {branches.map((branch: string, index) => {
                            const name = branch;
                            return (
                                <Option key={index} value={name}>
                                    {name}
                                </Option>
                            );
                        })}
                    </Select>

                    <div className={styles.repoPullText}>{t('repository.pull.create.pull')}</div>

                    <span className={styles.repoPullSpan}>
                        {t('repository.pull.create.traget.branch')}：
                    </span>
                    <Select
                        className={styles.repoPullSelect}
                        allowClear
                        onChange={(value) => onChange(value, 'target')}
                    >
                        {branches.map((branch: string, index) => {
                            const name = branch;
                            return (
                                <Option key={index} value={name}>
                                    {name}
                                </Option>
                            );
                        })}
                    </Select>
                </div>
                {diffResult == 1 && (
                    <div className={styles.repoPullHint}>There isn’t anything to compare.</div>
                )}
                {repositoryCompareInfo.diff.length === 0 && diffResult === 2 && (
                    <div className={styles.repoPullHint}>There isn’t anything to compare.</div>
                )}
                {repositoryCompareInfo.commits.length > 0 && diffResult != 1 && (
                    <RepoPullsCompare />
                )}
            </div>
        </div>
    );
};

export default withRouter(RepoPullsCreate);
