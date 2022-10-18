import React from 'react';
import { useHistory } from 'react-router-dom';
import { repositorysAtom } from '../../stores/repository';
import { useRecoilState } from 'recoil';
import PrivateImg from '@src/assets/images/private.png';
import OpenImg from '@src/assets/images/open.png';
import styles from './RepoPanelList.css';

const RepoPanelList: React.FC = () => {
    const [activeKey, setActiveKey] = React.useState(-1);
    const history = useHistory();
    const [repositorys] = useRecoilState(repositorysAtom);

    const disposeName = (name: string) => {
        return name.length <= 12 ? name : `${name.slice(0, 6)}...${name.slice(name.length - 6)}`;
    };

    const linkRepoDetail = (repo_name_origin: string) => {
        history.push(`/${repo_name_origin}`);
    };

    return (
        <div className={styles.repoListBlock}>
            {repositorys.map((item: ResponseRepository) => {
                const repo_name_origin = `${item.author_name}/${item.name}`;
                const repo_name = `${disposeName(item.author_name)}/${item.name}`;
                const imgUrl = item.is_private == 1 ? PrivateImg : OpenImg;
                return (
                    <div
                        className={styles.repoListItem}
                        key={repo_name_origin}
                        onClick={() => linkRepoDetail(repo_name_origin)}
                        // onMouseOver={()=>{ setActiveKey(index) }}
                        // onMouseOut={()=>{ setActiveKey(-1) }}
                    >
                        <img src={imgUrl} className={styles.repoListItemIcon} />
                        <div className={styles.repoListItemName} title={repo_name_origin}>
                            {repo_name}
                        </div>
                        {/* {
                        activeKey === index &&
                        <div className="repo-list-item-float">
                            <img src={ imgUrl } className="repo-list-item-icon" alt="" />
                            <span>{repo_name_origin}</span>
                        </div>
                    } */}
                    </div>
                );
            })}
        </div>
    );
};

export default RepoPanelList;
