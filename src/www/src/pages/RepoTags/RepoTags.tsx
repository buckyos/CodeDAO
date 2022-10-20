import { message } from 'antd';
import React from 'react';
import { RouteComponentProps, withRouter } from 'react-router-dom';
import { stackInfo } from '@src/utils/stack';
import { repositoryAtom } from '@src/stores/repository';
import { toYMDHMS, requestTarget, requestLocal } from '@src/utils';
import { useRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import TagsIcon from '@src/assets/images/tags.png';
import RepoTagTimeIcon from '@src/assets/images/repo_tag_time.png';
import RepoTagCommitIcon from '@src/assets/images/repo_tag_commit.png';
import RepoTagZipIcon from '@src/assets/images/repo_tag_zip.png';
import styles from './RepoTags.module.less';

const RepoTags: React.FC<RouteComponentProps> = ({ match, history }) => {
    const [activeKey, setActiveKey] = React.useState('2');
    const [releases, setReleases] = React.useState<RepositoryRelease[]>([]);
    const { owner, object_id } = match.params as RepoUrlParams;
    const [repository] = useRecoilState(repositoryAtom);
    const { t } = useTranslation();

    React.useEffect(() => {
        (async function () {
            await fetchData();
        })();
        return () => {
            message.destroy();
        };
    }, []);

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
        if (r.err) {
            console.log(' fetch  repo star failed');
            return;
        }
        console.log(r);
        setReleases(r.data!.data);
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
                        className={styles.repoTagsTabItem}
                        onClick={() => {
                            history.push(`/${owner}/${object_id}/releases`);
                        }}
                    >
                        Releases
                    </div>
                    <div
                        className={styles.repoTagsTabItemActive}
                        onClick={() => {
                            history.push(`/${owner}/${object_id}/tags`);
                        }}
                    >
                        Tags
                    </div>
                </div>
            </div>
            {activeKey == '2' && (
                <div>
                    <div className={styles.tagsList}>
                        <div className={styles.tagsListHead}>
                            <img src={TagsIcon} />
                            <span>Tags</span>
                        </div>
                        <div className={styles.tagsListMain}>
                            {releases.map((ff: RepositoryRelease, index: number) => (
                                <div className={styles.tagsListItem} key={index}>
                                    <div className={styles.tagsListItemLeft}>
                                        <div className={styles.tagsListItemVesion}>
                                            <span>{ff.tag_name}</span>
                                            {/* <span>...</span> */}
                                        </div>
                                        <div className={styles.tagsListItemInfo}>
                                            <div className={styles.tagsListItemCommon}>
                                                <img src={RepoTagTimeIcon} />
                                                <span>{toYMDHMS(ff.date)}</span>
                                            </div>
                                            <div className={styles.tagsListItemCommon}>
                                                <img src={RepoTagCommitIcon} />
                                                <span>{ff.commit_id}</span>
                                            </div>
                                            <div
                                                className={styles.tagsListItemPackage}
                                                onClick={() => {
                                                    donwloadFile(ff);
                                                }}
                                            >
                                                <img src={RepoTagZipIcon} />
                                                <span>zip</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};

export default withRouter(RepoTags);
