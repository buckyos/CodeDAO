import { repositoryAtom } from '../../../stores/repository';
import React from 'react';
import { useRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import styles from './EmptyRepository.css';

const EmptyRepository: React.FC = () => {
    const [repository] = useRecoilState(repositoryAtom);

    const data = repository;
    // const {name} = useUserNameById(data.owner.toString())
    const remote = `git remote add origin cyfs://${data.author_name}/${data.name}`;
    const { t } = useTranslation();

    return (
        <div className={styles.box}>
            <h4 className={styles.emptyRepoHeader}>{t('repository.empty.title')}</h4>
            <div className={styles.innerBox}>
                <div>
                    <div className={styles.emptyRepoH4}>{t('repository.empty.new.text')}</div>
                    <pre className={styles.emptyTips}>
                        <code>
                            {`touch README.md
git init
git add README.md
git commit -m "first commit"
${remote}
git push -u origin master`}
                        </code>
                    </pre>
                </div>
                <div>
                    <div className={styles.emptyRepoH4}>{t('repository.empty.push.text')}</div>
                    <pre className={styles.emptyTips}>
                        <code>
                            {`${remote}
git push -u origin master`}
                        </code>
                    </pre>
                </div>

                <div>
                    <h5 className={styles.subTitle}>PS: git-remote-cyfs problem</h5>
                    <div className={styles.remote}>If you meet the error: 
                        <pre className={styles.inLinePre}>git: 'remote-cyfs' is not a git command. See 'git --help'</pre>,
                    </div>
                    <div className={styles.remote}>
                        You need to install the binary
                        <pre className={styles.inLinePre}>git-remote-cyfs</pre> 
                        &nbsp;&nbsp;in client first.
                    </div>
                </div>
            </div>

        </div>
    );
};
export default EmptyRepository;
