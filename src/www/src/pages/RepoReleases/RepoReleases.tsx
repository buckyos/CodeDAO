import { message } from 'antd';
import React from 'react';
import { RouteComponentProps, withRouter } from 'react-router-dom';
import { stackInfo } from '@src/utils/stack';
import { repositoryAtom, repositoryReleaseCountAtom } from '../../stores/repository';
import { enCodePath, requestLocal, requestTarget } from '../../utils';
import { useRecoilState, useSetRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import TagIcon from '@src/assets/images/tag.png';
import RepoTagCommitIcon from '@src/assets/images/repo_tag_commit.png';
import ReleaseZipIcon from '@src/assets/images/release_zip.png';
import styles from './RepoReleases.module.less';

const RepoReleases: React.FC<RouteComponentProps> = ({ match, history }) => {
    const [activeKey, setActiveKey] = React.useState('1');
    const { owner, object_id } = match.params as RepoUrlParams;
    const [releases, setReleases] = React.useState<RepositoryRelease[]>([]);
    const [showBtn, setShowBtn] = React.useState(false);
    const [repository, setRepositoryData] = useRecoilState(repositoryAtom);
    const setReleaseCount = useSetRecoilState(repositoryReleaseCountAtom);
    const { t } = useTranslation();

    const changeKey = (key: string) => {
        setActiveKey(key);
    };

    React.useEffect(() => {
        (async function () {
            await fetchData();
            await getRepoMember();
        })();
    }, []);

    const getRepoMember = async () => {
        const req: RequestRepositoryMemberGet = {
            repository_id: object_id,
            user_id: stackInfo.owner.to_base_58()
        };
        const r = await requestTarget<{ data?: ResponseRepositoryMember }>(
            'repository/member/get',
            req,
            owner,
            object_id
        );
        if (r.err || r.data === undefined) {
            message.error('get user info failed');
            return;
        }
        if (r.data.data && (r.data.data.role === 'admin' || r.data.data.role === 'write')) {
            setShowBtn(true);
        }
        console.log('r.data.data----', r.data.data);
    };

    const fetchData = async () => {
        // 这个考虑放repo/home 里面
        const r = await requestTarget<{ data: RepositoryRelease[] }>(
            'repo/releases',
            {
                owner: owner,
                id: object_id,
                user_id: stackInfo.owner
            },
            owner,
            object_id
        );
        if (r.err || r.data === undefined) {
            console.log(' fetch  repo star failed');
            return;
        }
        console.log(r);
        setReleases(r.data.data);
        setReleaseCount((r.data.data || []).length); // 更新版本发布数量
    };

    const donwloadFile = async (release: RepositoryRelease) => {
        message.destroy();
        const hide = message.loading('下载中...', 0);
        const r = await requestLocal<{ data: { file_url: string } }>('repo/release/download', {
            file_id: release.file_id,
            repo_owner: owner,
            repo_id: object_id,
            repo_name: repository.name,
            tag_name: release.tag_name
        });
        console.log('r----', r);
        if (r.err || r.data === undefined) {
            console.log(' fetch  repo star failed');
            message.error(t('error.release.download'));
            return;
        }
        const url = r.data!.data.file_url;
        const x = new XMLHttpRequest();
        x.open('GET', url, true);
        x.responseType = 'blob';
        x.onload = function () {
            const url = window.URL.createObjectURL(x.response);
            const a = document.createElement('a');
            a.href = url;
            a.download = `${repository.name}_${release.tag_name}.zip`;
            a.click();
            hide();
        };
        x.send();
    };

    return (
        <div>
            <div className={styles.repoTagsHeader}>
                <div className={styles.repoTagsTab}>
                    <div
                        className={
                            activeKey == '1' ? styles.repoTagsTabItemActive : styles.repoTagsTabItem
                        }
                        onClick={() => {
                            changeKey('1');
                            history.push(`/${owner}/${object_id}/releases`);
                        }}
                    >
                        Releases
                    </div>
                    <div
                        className={
                            activeKey == '2' ? styles.repoTagsTabItemActive : styles.repoTagsTabItem
                        }
                        onClick={() => {
                            changeKey('2');
                            history.push(`/${owner}/${object_id}/tags`);
                        }}
                    >
                        Tags
                    </div>
                </div>
                <div className={styles.repoReleaseIssue}>
                    {showBtn && (
                        <button
                            className={styles.repoReleaseBtn}
                            onClick={() => {
                                history.push(`/${owner}/${object_id}/release/new`);
                            }}
                        >
                            {t('repository.file.release.new')}
                        </button>
                    )}
                </div>
            </div>
            <div className="repo-release-list">
                {releases.map((ff: RepositoryRelease, index: number) => (
                    <div key={index} className={styles.releaseListItem}>
                        <div className={styles.releaseListItemLeft}>
                            <div className={styles.releaseTag}>
                                <img src={TagIcon} />
                                <span>{ff.tag_name}</span>
                            </div>
                            <div className={styles.releaseCommit}>
                                <img src={RepoTagCommitIcon} />
                                <span>{ff.commit_id}</span>
                            </div>
                        </div>
                        <div className={styles.releaseListItemRight}>
                            <div className={styles.releaseListItemRightMain}>
                                <div className={styles.releaseListItemRightHead}>
                                    <div className={styles.releaseTitle}>
                                        <span>{ff.title}</span>
                                        <span className={styles.releaseDot}></span>
                                    </div>
                                    <div className={styles.releaseEdit}>
                                        {showBtn && (
                                            <button
                                                className={styles.releaseEditBtn}
                                                onClick={() => {
                                                    history.push(
                                                        `/${owner}/${object_id}/release/edit/${enCodePath(
                                                            ff.tag_name
                                                        )}`
                                                    );
                                                }}
                                            >
                                                {t('repository.file.tag.detail.edit')}
                                            </button>
                                        )}
                                    </div>
                                </div>
                                <div>{ff.content}</div>
                                <div className={styles.releaseDownload}>
                                    <div className={styles.releaseZipTitle}>
                                        {t('repository.file.tag.detail.download')}
                                    </div>
                                    <div
                                        className={styles.releaseZip}
                                        onClick={() => {
                                            donwloadFile(ff);
                                        }}
                                    >
                                        <img src={ReleaseZipIcon} />
                                        <span>{t('repository.file.tag.detail.code')}</span>
                                        <span>zip</span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default withRouter(RepoReleases);
