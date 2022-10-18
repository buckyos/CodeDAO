import React, { useEffect, useCallback } from 'react';
import { Dropdown, Menu } from 'antd';
import { DownOutlined, BranchesOutlined, ClockCircleOutlined } from '@ant-design/icons';
import {
    branchesAtom,
    repositoryAtom,
    repositoryChildPath,
    repositoryCommitCountAtom,
    repositoryCurrentBranchAtom,
    repositoryReleaseCountAtom
} from '../../../../stores/repository';
import RepoHomeCodeHeaderBranch from '../RepoHomeCodeBranch/RepoHomeCodeBranch';
import { useHistory, useParams } from 'react-router-dom';
import { useRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import ForkIcon from '@src/assets/images/fork.png';
import styles from './RepoHomeCodeHeader.css';

const RepoHomeCodeHeader: React.FC = () => {
    const { owner, object_id } = useParams<RepoUrlParams>();
    const history = useHistory();
    const [show, setShow] = React.useState(false);
    const [repository] = useRecoilState(repositoryAtom);
    const [branches] = useRecoilState(branchesAtom);
    const [childPath] = useRecoilState(repositoryChildPath);
    const [branch] = useRecoilState(repositoryCurrentBranchAtom);
    const [commitCount] = useRecoilState(repositoryCommitCountAtom);
    const [releaseCount] = useRecoilState(repositoryReleaseCountAtom);
    const { t } = useTranslation();

    const showCodeBtn = !['commits', 'branches', 'releases', 'release', 'tags', 'commit', 'blob']
        .map((ff) => location.hash.indexOf(ff) > -1)
        .includes(true);

    // const {name} = useUserNameById(data.owner.toString())
    const authorName = repository.author_name;
    const remote = `cyfs://${authorName}/${repository.name}`;

    useEffect(() => {
        function handleClick() {
            setShow(false);
        }
        document.addEventListener('click', handleClick);
        return () => document.removeEventListener('click', handleClick);
    }, []);

    const BranchDropdownList = () => (
        <Menu>
            {branches.map((branch: string) => (
                <Menu.Item
                    key={branch}
                    onClick={() => {
                        history.push(`/${owner}/${object_id}/src/${branch}`);
                    }}
                >
                    {branch}
                </Menu.Item>
            ))}
        </Menu>
    );

    const repoUrlBoX = () => (
        <div
            className={styles.repoUrlBox}
            onClick={(e) => {
                e.nativeEvent.stopImmediatePropagation();
                setShow(true);
            }}
        >
            {t('repository.file.address')}：
            <br />
            git clone {remote}
        </div>
    );

    const locationCommits = () => {
        history.push(`/${owner}/${object_id}/commits/${branch}`);
    };

    const gotoGraph = useCallback(() => history.push(`/${owner}/${object_id}/graph`), []);

    const detailHead = () => {
        return [
            <div key="graph" className={styles.repoDetailCommiItem} onClick={gotoGraph}>
                <BranchesOutlined className={styles.repoBranch} alt="" />
                <dd>{t('repository.file.graph')}</dd>
            </div>,
            <div key="commits" className={styles.repoDetailCommiItem} onClick={locationCommits}>
                <ClockCircleOutlined className={styles.repoBranch} alt="" />
                <span>{commitCount}</span>
                <dd>{t('repository.file.submit')}</dd>
            </div>,

            <RepoHomeCodeHeaderBranch key="branches" />,
            <div key="pad" style={{ flex: 1 }}></div>,
            showCodeBtn && (
                <div key="code clone" className={styles.repoUrl}>
                    <div
                        className={styles.repoUrlMain}
                        onClick={(e) => {
                            e.nativeEvent.stopImmediatePropagation();
                            setShow(!show);
                        }}
                    >
                        <span>Code</span>
                        <span></span>
                    </div>
                    {show && repoUrlBoX()}
                </div>
            )
        ];
    };

    return (
        <div className={styles.repoDetailCommit}>
            <div className={styles.repoDetailCommiItem}>
                <Dropdown
                    overlay={BranchDropdownList()}
                    placement="bottomCenter"
                    arrow
                    trigger={['click']}
                >
                    <div className={styles.branchSwitchDropdown}>
                        <img src={ForkIcon} />
                        <span style={{ marginRight: 10 }}>{t('repository.file.branch')}:</span>
                        <strong>{branch}</strong>
                        <DownOutlined style={{ marginLeft: 5, marginTop: 4 }} />
                    </div>
                </Dropdown>
            </div>
            {childPath && <div className={styles.repoDetailCommiItem}>{childPath}</div>}
            {childPath == '' && detailHead()}
        </div>
    );
};

export default RepoHomeCodeHeader;
