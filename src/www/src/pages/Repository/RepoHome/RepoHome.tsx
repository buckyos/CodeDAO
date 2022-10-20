import React, { useEffect } from 'react';
import { Switch, Route, useParams } from 'react-router-dom';
import { message, Spin } from 'antd';
import RepoDetailTabs from '../RepoHomeMenus/RepoHomeMenus';
import RepoIssue from '../Issue/RepoIssue/RepoIssue';
import RepoPulls from '../Merge/RepoPulls/RepoPulls';
import RepoPullsCreate from '../Merge/RepoPullsCreate/RepoPullsCreate';
import RepoPullsDetail from '../Merge/RepoPullsDetail/RepoPullsDetail';
import RepoSetting from '../Setting/RepoSetting/RepoSetting';
import {
    repositoryAtom,
    branchesAtom,
    repositoryCurrentBranchAtom,
    useCleanRepoHome,
    repositoryCommitCountAtom,
    repositoryReleaseCountAtom,
    repositoryLastCommitAtom,
    repositoryStarCountAtom,
    repositorySettingAtom
} from '@src/stores/repository';
import { requestLocal, requestTarget } from '@src/utils/index';
import RepoIssueNew from '../Issue/RepoIssueNew/RepoIssueNew';
import RepoIssueDetail from '../Issue/RepoIssueDetail/RepoIssueDetail';
import * as _ from 'lodash';
import { RepoWiki } from '../Wiki/RepoWiki/RepoWiki';
import { RepoWikiNew } from '../Wiki/RepoWikiNew/RepoWikiNew';
import RepoHomeHeader from '../RepoHomeHeader/RepoHomeHeader';
import { useRecoilState, useSetRecoilState } from 'recoil';
import RepoHomeCode from '../RepoHomeCode/RepoHomeCode';
import styles from './RepoHome.css';

const RepoHome: React.FC = () => {
    const { owner, object_id, branch: urlBranch } = useParams<RepoUrlParams>();
    const [repository, setRepositoryData] = useRecoilState(repositoryAtom);
    const [branch, setBranch] = useRecoilState(repositoryCurrentBranchAtom);
    const setBranches = useSetRecoilState(branchesAtom);
    const setLastCommit = useSetRecoilState(repositoryLastCommitAtom);
    const setCommitCount = useSetRecoilState(repositoryCommitCountAtom);
    const setReleaseCount = useSetRecoilState(repositoryReleaseCountAtom);
    const setStarCount = useSetRecoilState(repositoryStarCountAtom);
    const setRepositorySetting = useSetRecoilState(repositorySettingAtom);

    const cleanRepositoryHomeData = useCleanRepoHome();

    useEffect(() => {
        if (_.isEmpty(urlBranch)) {
            setBranch('master');
        } else {
            setBranch(urlBranch);
        }
    }, [urlBranch]);

    useEffect(() => {
        if (_.isEmpty(branch)) {
            return;
        }

        (async function () {
            const r = await requestLocal<ResponseRepositoryHome>('repo/home', {
                author_name: owner,
                name: object_id,
                branch: branch
            });
            if (r.err || r.data === undefined) {
                message.error(`fetch dec app data err ${r.msg}`);
                return;
            }
            const data = r.data;
            console.log('data---', data);
            setBranches(data.branches);
            // setBranch(branch)
            setLastCommit(data.last_commit!);
            setCommitCount(data.commit_count);
            setReleaseCount(data.releaseCount!);
            setRepositoryData(data.repository);
            setStarCount(data.star_count);
            setRepositorySetting(data.is_setting);
        })();
        return () => {
            cleanRepositoryHomeData();
        };
    }, [branch, object_id, owner]);

    if (!repository.id) {
        return (
            <div className={styles.detailContent}>
                <Spin />
            </div>
        );
    }

    return (
        <div className={styles.detailContent}>
            <RepoHomeHeader />
            <RepoDetailTabs />
            <Switch>
                <Route exact path="/:owner/:object_id/issues">
                    <RepoIssue />
                </Route>
                <Route exact path="/:owner/:object_id/issues/new">
                    <RepoIssueNew />
                </Route>
                <Route exact path="/:owner/:object_id/issues/:issue_id">
                    <RepoIssueDetail />
                </Route>
                <Route path="/:owner/:object_id/pulls/create/">
                    <RepoPullsCreate />
                </Route>
                <Route path="/:owner/:object_id/pulls/:pull_id">
                    <RepoPullsDetail />
                </Route>
                <Route path="/:owner/:object_id/pulls">
                    <RepoPulls />
                </Route>
                <Route path="/:owner/:object_id/setting">
                    <RepoSetting />
                </Route>
                <Route path="/:owner/:object_id/wiki/edit/:page_title">
                    <RepoWikiNew />
                </Route>
                <Route path="/:owner/:object_id/wiki/new">
                    <RepoWikiNew />
                </Route>
                <Route path="/:owner/:object_id/wiki/:page_title">
                    <RepoWiki />
                </Route>
                <Route path="/:owner/:object_id/wiki">
                    <RepoWiki />
                </Route>
                <Route path="/:owner/:object_id">
                    <RepoHomeCode />
                </Route>
            </Switch>
        </div>
    );
};

export default RepoHome;
