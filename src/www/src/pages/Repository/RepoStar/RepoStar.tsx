import React from 'react';
import { useParams } from 'react-router-dom';
import { stackInfo } from '@src/utils/stack';
import { repositoryAtom, repositoryStarCountAtom } from '@src/stores/repository';
import { message } from 'antd';
import { requestTarget, requestLocal } from '@src/utils/index';
import { useRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import { userInfoAtom } from '@src/stores/user';
import ForkIcon from '@src/assets/images/fork.png';
import EyeIcon from '@src/assets/images/eye.png';
import StarIcon from '@src/assets/images/star.png';
import StarCheckedIcon from '@src/assets/images/star_checked.png';
import styles from './RepoStar.css';

const RepoStar: React.FC = () => {
    // const [starNumber, setStarNumber] = React.useState(0)
    const [forkNumber, setForkNumber] = React.useState(0);
    const [hadStared, setHadStared] = React.useState(false);
    const { owner, object_id } = useParams<RepoUrlParams>();
    const [repository] = useRecoilState(repositoryAtom);
    const [star_count] = useRecoilState(repositoryStarCountAtom);
    const [userInfo] = useRecoilState(userInfoAtom);
    const { t } = useTranslation();

    // const fetchData = async () => {
    //     // 这个考虑放repo/home 里面
    //     const r = await requestTarget<ResponseRepositoryStar>('repo/star', {
    //         owner: owner,
    //         id: object_id,
    //         user_id: stackInfo.owner,
    //     }, owner, object_id)
    //     if (r.err || r.data === undefined) {
    //         console.log(' fetch  repo star failed')
    //         return
    //     }
    //
    //     setStarNumber(r.data.number)
    //     setHadStared(r.data.stared)
    //     setForkNumber(r.data.forkNumber)
    // }

    // React.useEffect(() => {
    //     (async function (){
    //         await fetchData()
    //     }())
    // }, [])

    const starSubmit = async () => {
        const r = await requestLocal('repo/star', {
            owner: owner,
            id: object_id,
            author_name: owner,
            name: object_id,
            user_id: stackInfo.owner,
            user_name: userInfo.name
        });
        if (r.err) {
            console.log(' fetch  repo star failed');
            message.error(r.msg);
            return;
        }
        message.success(t('success.repository.star.success'));
        // TODO 优化update
        setTimeout(() => {
            location.reload();
        }, 1200);
        // await fetchData()
    };

    // const unStar = async () => {
    //     const r = await requestTarget('repo/star/delete', {
    //         owner: owner,
    //         id: object_id,
    //         user_id:  stackInfo.owner,
    //     }, owner, object_id)
    //     if (r.err) {
    //         console.log(' fetch  repo star failed')
    //         message.error(r.msg)
    //         return
    //     }
    //     message.success(t('success.repository.star.cancel'))
    //     await fetchData()
    // }

    const onFork = async () => {
        message.info(t('repository.fork.develop'));
        return;
        // if (stackInfo.checkOwner(owner)) {
        //     message.error(`can't fork, because you own this repository`)
        //     return
        // }

        // if (!buttonStatus) {
        //     message.error(`正在fork...`)
        //     return
        // }

        // setButtonStatus(false)
        // const r = await requestTarget('repo/fork', {
        //     owner: owner,
        //     id: object_id,
        //     user_id:  stackInfo.owner.toString(),
        //     ood: stackInfo.ood_device_id.to_base_58(),
        // } as RequestRepositoryFork, owner, object_id)
        // if (r.err) {
        //     message.error(r.msg)
        //     return
        // }

        // message.success('fork仓库成功. 正在跳转...')
    };

    return (
        <div className={styles.repoDetailAction}>
            <div
                className={styles.repoDetailActionItem}
                onClick={() => message.info(t('repository.fork.develop'))}
            >
                <div>
                    <img src={EyeIcon} />
                    watch
                </div>
                <span>0</span>
            </div>
            <div className={styles.repoDetailActionItem} onClick={starSubmit}>
                {!hadStared && (
                    <div>
                        <img src={StarIcon} />
                        star
                    </div>
                )}
                {hadStared && (
                    <div>
                        <img src={StarCheckedIcon} />
                        unstar
                    </div>
                )}
                <span>{star_count}</span>
            </div>

            {!repository.fork_from && (
                <div className={styles.repoDetailActionItem} onClick={onFork}>
                    <div>
                        <img src={ForkIcon} />
                        fork
                    </div>
                    <span>{forkNumber}</span>
                </div>
            )}
        </div>
    );
};

export default RepoStar;
