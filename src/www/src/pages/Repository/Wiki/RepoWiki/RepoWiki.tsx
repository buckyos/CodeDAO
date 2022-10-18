import { message, Spin } from 'antd';
import React, { useEffect, useState } from 'react';
import { useHistory, useParams } from 'react-router-dom';
import { enCodePath, requestTarget } from '../../../../utils';
import style from './RepoWiki.css';
import { MdShow } from '../../../../components/MdShow/MdShow';
import { ResponseIssue } from '../../../../common/types';
import dayjs from 'dayjs';
import { useTranslation } from 'react-i18next';
import WikiDownIcon from '@src/assets/images/wiki_down.png';

export const PageNameDate: React.FC<{ page: ResponseRepoWikiPage }> = ({ page }) => {
    // if(!page.publisher_id){
    //     return ( <div></div> )
    // }
    // const name = useUserNameById(page.publisher_id).name
    // const fromNow = dayjs(page.date).fromNow()
    // console.log('edited this page on---', page.publisher_id, name)

    return <div>{/* <span>{name} created this page on {fromNow}</span> */}</div>;
};

export const RepoWiki: React.FC = () => {
    const { t } = useTranslation();

    return (
        <div style={{ textAlign: 'center', lineHeight: '40px' }}>
            {t('repository.wiki.develop')}
        </div>
    );

    // const { owner, object_id, page_title } = useParams<RepoWikiHomeRequest>();
    // const history = useHistory();
    // const [pages, setPages] = useState<ResponseRepoWikiPage[]>([]);
    // const [laoding, setLoading] = useState(true);
    // const [filterPages, setFilterPages] = useState<ResponseRepoWikiPage[]>([]);
    // const [page, setPage] = useState<ResponseRepoWikiPage>({
    //     content: '',
    //     date: 0,
    //     id: '',
    //     publisher_id: '',
    //     title: ''
    // });

    // useEffect(() => {
    //     (async function () {
    //         const param: { id: string; title?: string } = {
    //             id: object_id
    //         };
    //         if (page_title) {
    //             param.title = decodeURIComponent(page_title);
    //         }
    //         const r = await requestTarget<ResponseRepoWikiHome>(
    //             'repo/wiki/page/home',
    //             param,
    //             owner,
    //             object_id
    //         );
    //         console.log('r------', r);
    //         if (r.err || r.data!.data === undefined) {
    //             message.error(r.msg);
    //             console.log('get wiki page home failed');
    //             return;
    //         }
    //         setPages(r.data!.data);
    //         setFilterPages(r.data!.data);
    //         if (r.data!.page) {
    //             setPage(r.data!.page);
    //         }
    //         setLoading(false);
    //     })();
    // }, [page_title]);

    // const getFilterPages = (value: string) => {
    //     if (value) {
    //         setFilterPages(
    //             pages.filter((ff: ResponseRepoWikiPage) => ff.title.indexOf(value) > -1)
    //         );
    //     } else {
    //         setFilterPages(pages);
    //     }
    // };

    // if (laoding) {
    //     return <Spin />;
    // }

    // return (
    //     <div>
    //         <div className={style.repoWiki}>
    //             <div className={style.wikiHeader}>
    //                 <div className={style.wikiHeaderTop}>
    //                     <div className={style.wikiHeaderLeft}>{page.title}</div>
    //                     <div className="wiki-header-right">
    //                         {pages.length > 0 && (
    //                             <button
    //                                 className={style.wikiEditBtn}
    //                                 onClick={() => {
    //                                     history.push(
    //                                         `/${owner}/${object_id}/wiki/edit/${enCodePath(
    //                                             page.title
    //                                         )}`
    //                                     );
    //                                 }}
    //                             >
    // 								Edit
    //                             </button>
    //                         )}
    //                         <button
    //                             className={style.wikiNewBtn}
    //                             onClick={() => {
    //                                 history.push(`/${owner}/${object_id}/wiki/new`);
    //                             }}
    //                         >
    // 							New Page
    //                         </button>
    //                     </div>
    //                 </div>
    //                 <PageNameDate page={page} />
    //             </div>
    //             <div className={style.wikiMain}>
    //                 <div className={style.wikiLeft}>
    //                     <MdShow content={page.content} />
    //                 </div>
    //                 <div className={style.wikiRight}>
    //                     <div className="wiki-right-header">
    //                         <div className={style.wikiPage}>
    //                             <div className={style.wikiPageHeader}>
    //                                 <img src={WikiDownIcon} alt="" />
    //                                 <span className={style.wikiPageText}>Pages</span>
    //                                 <span className={style.wikiPageCount}>
    //                                     {filterPages.length}
    //                                 </span>
    //                             </div>
    //                             <div className={style.wikiPageSearch}>
    //                                 <input
    //                                     type="text"
    //                                     onChange={(e: any) => {
    //                                         getFilterPages(e.target.value);
    //                                     }}
    //                                     placeholder="Find a Page…"
    //                                 />
    //                             </div>
    //                             <div className="wiki-page-list">
    //                                 {filterPages.map((ff: ResponseRepoWikiPage, i: number) => (
    //                                     <div
    //                                         className={style.wikiPageItem}
    //                                         key={ff.id}
    //                                         onClick={() => {
    //                                             history.push(
    //                                                 `/${owner}/${object_id}/wiki/${enCodePath(ff.title)}`
    //                                             );
    //                                         }}
    //                                     >
    //                                         {ff.title}
    //                                     </div>
    //                                 ))}
    //                             </div>
    //                         </div>
    //                         {/* <div className={ style["wiki-add-footer"] }>
    //                             <span></span>
    //                             <img src="" alt="" />
    //                             <span>Add a custom footter</span>
    //                         </div>                         */}
    //                     </div>
    //                 </div>
    //             </div>
    //         </div>
    //     </div>
    // );
};
