import MdEditor from 'react-markdown-editor-lite';
import MarkdownIt from 'markdown-it';
import React from 'react';
import styles from './MdShow.module.less';

const mdParser = new MarkdownIt({
    breaks: true
});
export const MdShow: React.FC<{ content: string }> = ({ content }) => {
    return (
        <div className={styles.mdBox}>
            <MdEditor
                defaultValue={''}
                style={{ height: 'auto' }}
                value={content}
                plugins={[]}
                autoFocus={false}
                readOnly={true}
                renderHTML={(text: string) => mdParser.render(text)}
            />
        </div>
    );
};
