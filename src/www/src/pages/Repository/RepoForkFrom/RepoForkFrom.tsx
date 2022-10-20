import React from 'react';
import { repositoryAtom } from '@src/stores/repository';
import { useHistory } from 'react-router-dom';
import { useRecoilState } from 'recoil';
import styles from './RepoForkFrom.css';

const RepoForkFrom: React.FC = () => {
    const [repository] = useRecoilState(repositoryAtom);
    const history = useHistory();

    if (!repository.fork_from && repository.fork_repository === undefined) {
        return null;
    }

    const fork_repository = repository.fork_repository!;

    const locationRepository = () => {
        history.push(`/${fork_repository.owner}/${fork_repository.id}`);
    };

    return (
        <div className={styles.repoDetailForkFrom}>
            <span>fork from</span>
            <span onClick={locationRepository} className={styles.repoDetailForkFromLink}>
                {fork_repository.owner}/{fork_repository.name}
            </span>
        </div>
    );
};

export default RepoForkFrom;
