import React from 'react';
import styles from './CommitDiff.module.less';
import { DiffLineData } from '../../common/types';

const CommitDiff: React.FC<CommitDiffProps> = ({ diffData, header = true }) => {
    return (
        <div>
            {diffData.map((ff: LineData, i: number) => (
                <div className={styles.diffListItem} key={i}>
                    {header && <div className={styles.diffListItemHeader}>{ff.file_name}</div>}

                    <div className={styles.codeList}>
                        {/* <div className="tag-code">{ff.title}</div>*/}
                        <div className={styles.codeContentList}>
                            <table>
                                <tbody>
                                    {ff.file_content.map((ff: LineContent, j: number) => {
                                        let cls = '';
                                        let symbol = '';
                                        switch (ff.diff_type) {
                                            case 'remove':
                                                cls = styles.deleteCode;
                                                symbol = '-';
                                                break;
                                            case 'add':
                                                cls = styles.addCode;
                                                symbol = '+';
                                                break;
                                            case 'tag':
                                                cls = styles.lineTag;
                                                break;
                                            default:
                                                cls = styles.normalCode;
                                        }
                                        return (
                                            <tr className={cls} key={j}>
                                                <td className={styles.lineCount}>{ff.left_line}</td>
                                                <td className={styles.lineCount}>
                                                    {ff.right_line}
                                                </td>
                                                <td style={{ width: 20, textAlign: 'center' }}>
                                                    {symbol}
                                                </td>
                                                <td>
                                                    <pre>{ff.content}</pre>
                                                </td>
                                            </tr>
                                        );
                                    })}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>
            ))}
        </div>
    );
};

export default CommitDiff;
