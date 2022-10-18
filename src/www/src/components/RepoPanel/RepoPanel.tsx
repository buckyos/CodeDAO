import React, { useState } from 'react';
import { useHistory } from 'react-router-dom';
import { message } from 'antd';
import { repositorysAtom } from '../../stores/repository';
import RepoPanelList from '../RepoPanelList/RepoPanelList';
import { requestLocal } from '../../utils/index';
import { useRecoilState } from 'recoil';
import { useTranslation } from 'react-i18next';
import BookIcon from '@src/assets/images/book.png';
import HomeSearchIcon from '@src/assets/images/home_search.png';
import styles from './RepoPanel.css';

const RepoPanel: React.FC = () => {
    const history = useHistory();
    const [repoName, setRepoName] = useState('');
    const [pageIndex, setPageIndex] = useState(0);
    const [search, setSearch] = useState(false);
    const [repositorys, setRepositorys] = useRecoilState(repositorysAtom);
    const [total, setTotal] = useState(0);
    const pageSize = 10;
    const { t } = useTranslation();

    const getRepoList = async (
        repoName: string,
        pageIndex: number,
        pageSize: number,
        isSearch: boolean | undefined
    ) => {
        const param: { repo_name?: string } = {};
        if (repoName) {
            param.repo_name = repoName;
        }

        const r = await requestLocal<{ data: ResponseRepository[]; count: number }>('repo/list', {
            ...param,
            page_index: pageIndex,
            page_size: pageSize
        });
        if (r.err) {
            message.error(t('error.home.repository.fail'));
            return;
        }
        console.log('request local', r);
        if (r.data) {
            const data =
                isSearch || pageIndex === 0 ?
                    r.data.data :
                    Array.from(new Set(repositorys.concat(r.data.data)));
            setRepositorys(data);
            setTotal(r.data!.count);
        }
    };

    React.useEffect(() => {
        (async function () {
            await getRepoList(repoName, pageIndex, pageSize, search);
        })();
    }, [pageIndex]);

    const linkCreateRepo = () => {
        history.push('/create/repo');
    };

    const searchRepo = async () => {
        setSearch(true);
        setPageIndex(0);
        await getRepoList(repoName, pageIndex, pageSize, search);
    };

    const showMoreRepo = () => {
        const totalPage = Math.ceil(total / pageSize) - 1;
        const index = pageIndex + 1 >= totalPage ? totalPage : pageIndex + 1;
        setSearch(false);
        setPageIndex(index);
    };

    return (
        <div className={styles.contentLeft}>
            <div className={styles.contentLeftHead}>
                <span className={styles.contentLeftTitle}>{t('home.repositorys')}</span>
                <div className={styles.newRepo} onClick={linkCreateRepo}>
                    <img src={BookIcon} />
                    <span>{t('home.repository.new')}</span>
                </div>
            </div>
            <div className={styles.searchRepoWrap}>
                <input
                    className={styles.searchRepo}
                    placeholder={t('home.repository.serach.input')}
                    onChange={(e) => {
                        setRepoName(e.target.value);
                    }}
                />
                <span className={styles.searchRepoSpan} onClick={searchRepo}>
                    <img src={HomeSearchIcon} />
                </span>
            </div>
            <div className={styles.repoListWrap}>
                <RepoPanelList />
            </div>
            {/* <div className={styles.showMoreText} onClick={showMoreRepo}>
                {t('home.repository.show.more')}
            </div> */}
        </div>
    );
};

export default RepoPanel;
