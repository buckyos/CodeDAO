import React from 'react';
import { useRecoilState } from 'recoil';
import { branchesAtom } from '@src/stores/repository';
import styles from './RepoBranches.css';

const RepoBranches: React.FC = () => {
    const [branches] = useRecoilState(branchesAtom);

    return (
        <div className={styles.repoBranches}>
            <div className={styles.repoBranchesHead}>所有分支</div>
            <div className={styles.repoBranchesMain}>
                {branches.map((ff) => (
                    <div className={styles.repoBranchesItem} key={ff}>
                        {ff}
                    </div>
                ))}
            </div>
        </div>
    );
};

export default RepoBranches;
