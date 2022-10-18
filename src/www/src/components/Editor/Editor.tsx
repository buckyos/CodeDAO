import React, { useCallback, useEffect } from 'react';
import { Tooltip } from 'antd';
import {
    BoldOutlined,
    CheckSquareOutlined,
    CodeOutlined,
    FontSizeOutlined,
    ItalicOutlined,
    LinkOutlined,
    MessageOutlined,
    OrderedListOutlined,
    PicRightOutlined,
    SelectOutlined,
    UnorderedListOutlined
} from '@ant-design/icons';
import styles from './Editor.css';
import MdEditor from 'react-markdown-editor-lite';
import { classSet } from '../../utils';
import MarkdownIt from 'markdown-it';
import { useTranslation } from 'react-i18next';

const mdParser = new MarkdownIt({
    breaks: true
});

function useEditorTool(
    ref: React.RefObject<HTMLTextAreaElement>,
    insert: string[],
    setFunc: React.Dispatch<React.SetStateAction<string>>,
    deps: React.DependencyList
) {
    return useCallback(() => {
        if (ref.current == null) {
            return;
        }
        const source = ref.current as HTMLTextAreaElement;
        const start = source.selectionStart;
        const end = source.selectionEnd;
        const oldContent = source.value || '';
        const text = oldContent.slice(start, end);

        const beforeContent = oldContent.slice(0, start);

        // replaceSelected
        let afterContent = '';
        if (insert[1].length > 0 || end > start) {
            afterContent = oldContent.slice(end, oldContent.length);
        } else {
            afterContent = oldContent.slice(start, oldContent.length);
        }

        // console.log(beforeContent, afterContent)
        // console.log(source.value, start,end ,text)
        const wrap = insert[0] + text + insert[1];
        const newText = beforeContent + wrap + afterContent;

        const addTextLength = insert[0].length;

        setTimeout(() => {
            source.setSelectionRange(start + addTextLength, end + addTextLength, 'forward');
            source.focus();
        });
        setFunc(newText);
    }, deps);
}

const Editor: React.FC<EditorProps> = ({ onChange, value }) => {
    const ref = React.useRef<HTMLTextAreaElement>(null);
    const [activeKey, setActiveKey] = React.useState('1');
    const [content, setContent] = React.useState('');
    const [showToolsMenu, setShowToolsMenu] = React.useState(true);
    const { t } = useTranslation();

    const onEdit = useCallback(() => {
        setActiveKey('1');
        setShowToolsMenu(true);
    }, []);
    const onPreview = useCallback(() => {
        setActiveKey('2');
        setShowToolsMenu(false);
    }, []);
    const onInnerChange = useCallback((event: React.ChangeEvent<HTMLTextAreaElement>) => {
        setContent(event.target.value);
    }, []);

    useEffect(() => {
        onChange(content);
    }, [content]);

    const addTitle = useEditorTool(ref, ['### ', ''], setContent, []);
    const addBold = useEditorTool(ref, ['**', '**'], setContent, []);
    const addItalic = useEditorTool(ref, ['_', '_'], setContent, []);
    const addQuota = useEditorTool(ref, ['> ', ''], setContent, []);
    const addCode = useEditorTool(ref, ['```\n', '\n```\n'], setContent, []);
    const addUrl = useEditorTool(ref, ['[', '](url)'], setContent, []);
    const addUnorderedList = useEditorTool(ref, ['- ', ''], setContent, []);
    const addOrderedList = useEditorTool(ref, ['1. ', ''], setContent, []);
    const addTask = useEditorTool(ref, ['- [ ] ', ''], setContent, []);

    return (
        <div>
            <div className={styles.head}>
                <div
                    className={classSet([styles.switch, { [styles.active]: activeKey == '1' }])}
                    onClick={onEdit}
                >
                    {t('editor.edit')}
                </div>
                <div
                    className={classSet([styles.switch, { [styles.active]: activeKey == '2' }])}
                    onClick={onPreview}
                >
                    {t('editor.preview')}
                </div>

                {showToolsMenu && (
                    <div className={styles.icons}>
                        <Tooltip placement="topLeft" title="Add header text">
                            <FontSizeOutlined onClick={addTitle} />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Add bold text">
                            <BoldOutlined onClick={addBold} />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Add italic text">
                            <ItalicOutlined onClick={addItalic} />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Insert a quote">
                            <PicRightOutlined onClick={addQuota} />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Insert a code">
                            <CodeOutlined onClick={addCode} />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Add a link">
                            <LinkOutlined onClick={addUrl} />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Add a bulleted list">
                            <OrderedListOutlined onClick={addOrderedList} />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Add a numbered list">
                            <UnorderedListOutlined onClick={addUnorderedList} />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Add a task list">
                            <CheckSquareOutlined onClick={addTask} />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Directly mention a user or team">
                            <MessageOutlined />
                        </Tooltip>
                        <Tooltip placement="topLeft" title="Reference an issue, merge request">
                            <SelectOutlined />
                        </Tooltip>
                    </div>
                )}
            </div>

            <div className={styles.wrap}>
                {activeKey == '1' && (
                    <textarea
                        ref={ref}
                        value={content || value}
                        className={styles.textarea}
                        rows={8}
                        onChange={onInnerChange}
                    />
                )}
                {activeKey == '2' && (
                    <div className={styles.wrap}>
                        {/* <ReactMarkdown children={content}/> */}
                        <MdEditor
                            style={{ height: 'auto' }}
                            value={content || value}
                            plugins={[]}
                            autoFocus={false}
                            readOnly={true}
                            renderHTML={(text) => mdParser.render(text)}
                        />
                    </div>
                )}
            </div>
        </div>
    );
};

export default Editor;
