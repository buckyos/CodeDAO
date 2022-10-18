import { message } from 'antd';
import React, { useEffect, useRef, useState } from 'react';
import { useHistory, useParams } from 'react-router-dom';
import { stackInfo } from '@src/utils/stack';
import Editor from '../../../../components/Editor/Editor';
import PromptDialog from '../../../../components/PromptDialog';
import { checkName, requestTarget } from '../../../../utils';
import styles from './RepoWikiNew.css';
import { useTranslation } from 'react-i18next';

export const RepoWikiNew: React.FC = () => {
    const { t } = useTranslation();
    const history = useHistory();
    const [content, setContent] = useState('');
    const [title, setTitle] = useState('');
    const [isShowDelete, setIsShowDelete] = useState(false);
    const childCmpRef = useRef<PromptDialogRefType>(null);
    const [deleteParam, setParam] = useState<DeleteParam>({ title: '', content: '' });
    const [page, setPage] = useState<ResponseRepoWikiPage>({
        content: '',
        date: 0,
        id: '',
        publisher_id: '',
        title: ''
    });
    const { owner, object_id, page_title } = useParams<RepoWikiParams>();
    const isEdit = location.href.indexOf('edit') > -1;
    const text = isEdit ? 'Edit page' : 'Create new page';
    console.log('isEdit---', isEdit, page_title);

    useEffect(() => {
        if (isEdit) {
            (async function () {
                await getWikiPageDetailData(decodeURIComponent(page_title));
            })();
        }
    }, []);

    const getWikiPageDetailData = async (title: string) => {
        const r = await requestTarget<{ data: ResponseRepoWikiPage }>(
            'repo/wiki/page/detail',
            {
                id: object_id,
                title
            },
            owner,
            object_id
        );
        if (r.err || r.data!.data === undefined) {
            message.error(t('error.wiki.new.detail.fail'));
            return;
        }
        const data = r.data!.data;
        setPage(data);
        setTitle(data.title);
        setContent(data.content);
        setIsShowDelete(stackInfo.owner.toString() === data.publisher_id);
        console.log('detail----r----', r, stackInfo.owner.toString(), data.publisher_id);
    };

    const submitPageContent = async () => {
        if (title.trim() === '') {
            message.warn(t('warn.wiki.new.title'));
            return;
        }
        if (!checkName(title.trim())) {
            message.warn(t('warn.wiki.new.title.format'));
            return;
        }
        if (content === '') {
            message.warn(t('warn.wiki.new.content'));
            return;
        }
        if (isEdit) {
            await repoWikiPageEdit();
            return;
        }
        await repoWikiPageNew();
    };

    const repoWikiPageNew = async () => {
        const r = await requestTarget(
            'repo/wiki/page/new',
            {
                owner: owner,
                id: object_id,
                user_id: stackInfo.owner,
                title,
                content
            },
            owner,
            object_id
        );
        if (r.err) {
            message.error(r.msg);
            console.log('new wiki page failed');
            return;
        }
        message.success(t('success.wiki.new'));
        setTimeout(() => {
            history.push(`/${owner}/${object_id}/wiki`);
        }, 1500);
        console.log(r);
    };

    const repoWikiPageEdit = async () => {
        const r = await requestTarget(
            'repo/wiki/page/edit',
            {
                id: object_id,
                wiki_id: page.id,
                title: title.trim(),
                content
            },
            owner,
            object_id
        );
        if (r.err) {
            message.error(r.msg);
            console.log('edit wiki page failed');
            return;
        }
        message.success(t('success.wiki.edit'));
        setTimeout(() => {
            history.push(`/${owner}/${object_id}/wiki`);
        }, 1500);
    };

    const repoWikiDelete = async () => {
        const r = await requestTarget(
            'repo/wiki/page/delete',
            {
                wiki_id: page.id
            },
            owner,
            object_id
        );
        if (r.err) {
            message.error(r.msg);
            console.log('delete wiki page failed');
            return;
        }
        successPrompt(t('success.wiki.delete'));
        setTimeout(() => {
            history.push(`/${owner}/${object_id}/wiki`);
        }, 1500);
    };

    const successPrompt = (text: string) => {
        childCmpRef.current?.setShow(false);
        childCmpRef.current?.setLoading(false);
        message.success(text);
    };

    const repositoryOperation = (title: string, content: string, callback: Function) => {
        childCmpRef.current?.setLoading(false);
        childCmpRef.current?.setShow(true);
        setParam({
            title: title,
            content: content,
            cb: callback
        });
    };

    const deleteSubmit = () => {
        repositoryOperation(
            t('wiki.detail.delete.prompt'),
            `${t('wiki.detail.delete.text')}" ${page.title}"？`,
            repoWikiDelete
        );
    };

    console.log('title---', text);

    return (
        <div className={styles.wikiNew}>
            <div>
                <div className={styles.wikiNewHeader}>
                    <div className={styles.headerLeft}>{text}</div>
                    <div className={styles.headerRight}>
                        {/* <button className="history-btn"> Page History </button> */}
                        {isShowDelete && (
                            <button
                                className={styles.deleteBtn}
                                onClick={() => {
                                    deleteSubmit();
                                }}
                            >
                                Delete Page
                            </button>
                        )}
                    </div>
                </div>
                <input
                    className={styles.wikiTitleInput}
                    type="text"
                    onChange={(e) => {
                        setTitle(e.target.value);
                    }}
                    value={title}
                    placeholder="请输入标题"
                />
            </div>
            <div className={styles.wikiNewEditor}>
                <Editor onChange={setContent} value={content} />
            </div>
            {/* <div className={ styles.wikiEditMessage }>
                <div className={ styles.wikiMessageTitle }>Edit message</div>
                <input className={ styles.wikiMessageInput } type="text" onChange={ (e) => { setContent(e.target.value) } } placeholder="请输入内容"/>
            </div> */}
            <div className={styles.wikiNewBottom}>
                <button
                    className={styles.wikiNewButton}
                    onClick={() => {
                        submitPageContent();
                    }}
                >
                    {t('wiki.detail.new.submit')}
                </button>
            </div>
            <PromptDialog param={deleteParam!} cRef={childCmpRef}></PromptDialog>
        </div>
    );
};
