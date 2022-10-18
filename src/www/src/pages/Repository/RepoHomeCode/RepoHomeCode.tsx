import React from 'react';
import RepoHomeCodeHeader from '../Code/RepoHomeCodeHeader/RepoHomeCodeHeader';
import { Route, Switch } from 'react-router-dom';
import { useRecoilState } from 'recoil';
import { repositoryAtom } from '../../../stores/repository';
import EmptyRepository from '../EmptyRepository/EmptyRepository';
import RepoTree from '../RepoTree/RepoTree';
import RepoFile from '../../RepoFile/RepoFile';
import RepoCommits from '../../RepoCommits/RepoCommits';
import RepoDiff from '../Diff/RepoDiff';
import RepoBranches from '../RepoBranches/RepoBranches';
import RepoGraph from '../Graph/RepoGroph';
import RepoTags from '../../RepoTags/RepoTags';
import RepoReleases from '../../RepoReleases/RepoReleases';
import RepoReleaseNew from '../../RepoReleaseNew/RepoReleaseNew';
import { RepoLanguage } from '../Language/RepoLanguage';
import { useTranslation } from 'react-i18next';
import styles from './RepoHomeCode.css';

const RepoHomeCode: React.FC = () => {
    const [repository] = useRecoilState(repositoryAtom);
    const { t } = useTranslation();

    const RepoCodeRight = () => (
        <div className={styles.detailMainRight}>
            <div>
                {t('repository.file.description')}：
                {repository.description == '' ?
                    t('repository.file.description.empty') :
                    repository.description}
            </div>
            <RepoLanguage />
        </div>
    );

    if (repository.init === 0) {
        return (
            <div className={styles.detailMain}>
                <div className={styles.detailMainLeft}>
                    <EmptyRepository />
                </div>
            </div>
        );
    }

    return (
        <div className={styles.detailMain}>
            <div className={styles.detailMainLeft}>
                <RepoHomeCodeHeader />
                <Switch>
                    <Route exact path="/:owner/:object_id">
                        <RepoTree />
                    </Route>
                    <Route exact path="/:owner/:object_id/src/:branch">
                        <RepoTree />
                    </Route>
                    <Route path="/:owner/:object_id/tree/:branch/">
                        <RepoTree />
                    </Route>
                    <Route path="/:owner/:object_id/tree/:branch/:file">
                        <RepoTree />
                    </Route>
                    <Route path="/:owner/:object_id/blob/:branch/:file">
                        <RepoFile />
                    </Route>
                    <Route exact path="/:owner/:object_id/commits/:branch">
                        <RepoCommits />
                    </Route>
                    <Route path="/:owner/:object_id/commit/:hashId">
                        <RepoDiff />
                    </Route>
                    <Route exact path="/:owner/:object_id/graph">
                        <RepoGraph />
                    </Route>
                    <Route exact path="/:owner/:object_id/branches">
                        <RepoBranches />
                    </Route>
                    <Route exact path="/:owner/:object_id/tags">
                        <RepoTags />
                    </Route>
                    <Route exact path="/:owner/:object_id/releases">
                        <RepoReleases />
                    </Route>
                    <Route exact path="/:owner/:object_id/release/new">
                        <RepoReleaseNew />
                    </Route>
                    <Route exact path="/:owner/:object_id/release/edit/:tag_name">
                        <RepoReleaseNew />
                    </Route>
                </Switch>
            </div>
            <Switch>
                <Route exact path="/:owner/:object_id">
                    {RepoCodeRight()}
                </Route>
                <Route exact path="/:owner/:object_id/src/:branch">
                    {RepoCodeRight()}
                </Route>
                <Route exact path="/:owner/:object_id/tree/:branch/">
                    {RepoCodeRight()}
                </Route>
            </Switch>
        </div>
    );
};

export default RepoHomeCode;
