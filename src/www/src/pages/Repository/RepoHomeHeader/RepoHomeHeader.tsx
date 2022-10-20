import RepoForkFrom from '../RepoForkFrom/RepoForkFrom';
import RepoStar from '../RepoStar/RepoStar';
import React from 'react';
import { repositoryAtom } from '@src/stores/repository';
import { useRecoilState } from 'recoil';
import PrivateImg from '@src/assets/images/private.png';
import OpenImg from '@src/assets/images/open.png';
import styles from './RepoHomeHeader.css';

const RepoHomeHeader: React.FC = () => {
    const [repository] = useRecoilState(repositoryAtom);

    return (
        <div className={styles.repoDetailHeader}>
            <div className={styles.repoDetailHeaderLeft}>
                <div className={styles.repoDetailHeaderName}>
                    {repository.is_private == 1 ? <img src={PrivateImg} /> : <img src={OpenImg} />}
                    <div className={styles.repoDetailName}>
                        {repository.author_name}/{repository.name}
                    </div>
                </div>
                <RepoForkFrom />
            </div>
            <RepoStar />
        </div>
    );
};

export default RepoHomeHeader;
