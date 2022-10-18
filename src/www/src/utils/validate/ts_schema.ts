export const schema = {
    '$schema': 'http://json-schema.org/draft-07/schema#',
    'definitions': {
        'TreeFileType': {
            'enum': [
                0,
                1
            ],
            'type': 'number'
        },
        'RepositoryType': {
            'enum': [
                0,
                1
            ],
            'type': 'number'
        },
        'AuthorType': {
            'enum': [
                'org',
                'user'
            ],
            'type': 'string'
        },
        'SetStateType': {
            'enum': [
                'setprivate',
                'setpublic'
            ],
            'type': 'string'
        },
        'global.ResponseGitLsTree': {
            'type': 'object',
            'properties': {
                'type': {
                    '$ref': '#/definitions/TreeFileType'
                },
                'treeData': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'fileMode': {
                                'type': 'string'
                            },
                            'gitObjectType': {
                                'type': 'string'
                            },
                            'fileHash': {
                                'type': 'string'
                            },
                            'fileName': {
                                'type': 'string'
                            }
                        },
                        'required': [
                            'fileHash',
                            'fileMode',
                            'fileName',
                            'gitObjectType'
                        ]
                    }
                },
                'content': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'type'
            ]
        },
        'global.ResponseGitLsTreeItem': {
            'type': 'object',
            'properties': {
                'fileMode': {
                    'type': 'string'
                },
                'gitObjectType': {
                    'type': 'string'
                },
                'fileHash': {
                    'type': 'string'
                },
                'fileName': {
                    'type': 'string'
                }
            },
            'required': [
                'fileHash',
                'fileMode',
                'fileName',
                'gitObjectType'
            ]
        },
        'global.ResponseOrganizationList': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                },
                'email': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                },
                'ownerId': {
                    'type': 'string'
                }
            },
            'required': [
                'date',
                'email',
                'id',
                'name',
                'ownerId'
            ]
        },
        'global.ResponseOrganizationHome': {
            'allOf': [
                {
                    'type': 'object',
                    'properties': {
                        'id': {
                            'type': 'string'
                        },
                        'name': {
                            'type': 'string'
                        },
                        'email': {
                            'type': 'string'
                        },
                        'date': {
                            'type': 'number'
                        },
                        'ownerId': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'date',
                        'email',
                        'id',
                        'name',
                        'ownerId'
                    ]
                },
                {
                    'type': 'object',
                    'properties': {
                        'memberCount': {
                            'type': 'number'
                        }
                    },
                    'required': [
                        'memberCount'
                    ]
                }
            ]
        },
        'global.RequestOrganizationMember': {
            'type': 'object',
            'properties': {
                'organization_id': {
                    'type': 'string'
                }
            },
            'required': [
                'organization_id'
            ]
        },
        'global.RequestOrganizationRepository': {
            'type': 'object',
            'properties': {
                'organization_name': {
                    'type': 'string'
                }
            },
            'required': [
                'organization_name'
            ]
        },
        'global.RequestOrganizationCheckRepositoryName': {
            'type': 'object',
            'properties': {
                'repository_name': {
                    'type': 'string'
                },
                'organization_id': {
                    'type': 'string'
                },
                'creator_id': {
                    'type': 'string'
                }
            },
            'required': [
                'organization_id',
                'repository_name'
            ]
        },
        'global.ResponseOrganizationCheckRepositoryName': {
            'type': 'object',
            'properties': {
                'nameExist': {
                    'type': 'boolean'
                }
            },
            'required': [
                'nameExist'
            ]
        },
        'global.RequestOrganizationMemberAdd': {
            'type': 'object',
            'properties': {
                'organization_id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'organization_id',
                'user_id'
            ]
        },
        'global.ResponseOrganizationMember': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                },
                'organization_id': {
                    'type': 'string'
                },
                'role': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'name',
                'organization_id',
                'role',
                'user_id'
            ]
        },
        'global.ResponseFriend': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'id'
            ]
        },
        'global.RequestNameById': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'id'
            ]
        },
        'global.RequestOnlyOwnerID': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                }
            },
            'required': [
                'owner'
            ]
        },
        'global.ResponseOptionsAuthor': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                },
                'type': {
                    '$ref': '#/definitions/AuthorType'
                }
            },
            'required': [
                'id',
                'name',
                'type'
            ]
        },
        'global.RequestRepositoryNew': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'author_id': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                },
                'description': {
                    'type': 'string'
                },
                'is_private': {
                    '$ref': '#/definitions/RepositoryType'
                },
                'author_type': {
                    '$ref': '#/definitions/AuthorType'
                },
                'author_name': {
                    'type': 'string'
                }
            },
            'required': [
                'author_id',
                'author_name',
                'author_type',
                'description',
                'is_private',
                'name',
                'owner'
            ]
        },
        'global.ResponseRepository': {
            '$ref': '#/definitions/ResponseRepository'
        },
        'global.ResponseRepositoryCommit': {
            'type': 'object',
            'properties': {
                'author': {
                    'type': 'string'
                },
                'message': {
                    'type': 'string'
                },
                'commit': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                }
            },
            'required': [
                'author',
                'commit',
                'date',
                'message'
            ]
        },
        'global.ServiceResponseUserData': {
            'type': 'object',
            'properties': {
                'userId': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                },
                'email': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                },
                'owner': {
                    'type': 'string'
                }
            },
            'required': [
                'date',
                'email',
                'name',
                'userId'
            ]
        },
        'global.ResponseUserGetIdByName': {
            'type': 'object',
            'properties': {
                'ownerId': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'type': {
                    'type': 'string'
                },
                'message': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'message',
                'ownerId',
                'type'
            ]
        },
        'global.RequestRepositoryMemberGet': {
            'type': 'object',
            'properties': {
                'repository_id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'repository_id',
                'user_id'
            ]
        },
        'global.RequestRepositoryMemberAdd': {
            'type': 'object',
            'properties': {
                'repository_id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'repository_id',
                'user_id'
            ]
        },
        'global.RequestRepositoryMemberList': {
            'type': 'object',
            'properties': {
                'repository_id': {
                    'type': 'string'
                }
            },
            'required': [
                'repository_id'
            ]
        },
        'global.RequestRepositoryMemberDelete': {
            'allOf': [
                {
                    'type': 'object',
                    'properties': {
                        'repository_id': {
                            'type': 'string'
                        },
                        'user_id': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'repository_id',
                        'user_id'
                    ]
                },
                {
                    'type': 'object',
                    'properties': {
                        'id': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'id'
                    ]
                }
            ]
        },
        'global.RequestRepositoryMemberChangeRole': {
            'allOf': [
                {
                    'type': 'object',
                    'properties': {
                        'repository_id': {
                            'type': 'string'
                        },
                        'user_id': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'repository_id',
                        'user_id'
                    ]
                },
                {
                    'type': 'object',
                    'properties': {
                        'id': {
                            'type': 'string'
                        },
                        'role': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'id',
                        'role'
                    ]
                }
            ]
        },
        'global.RequestRepositoryVerify': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'user_id'
            ]
        },
        'global.ResponseRepositoryMember': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'repository_id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                },
                'role': {
                    'type': 'string'
                },
                'user_name': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'repository_id',
                'role',
                'user_id',
                'user_name'
            ]
        },
        'global.ResponseRepositoryPushHead': {
            'type': 'object',
            'properties': {
                'refs': {
                    'type': 'string'
                }
            },
            'required': [
                'refs'
            ]
        },
        'global.ResponseRepositoryFetchHead': {
            'type': 'object',
            'properties': {
                'refs': {
                    'type': 'string'
                }
            },
            'required': [
                'refs'
            ]
        },
        'global.RequestRepositoryFetchHead': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                },
                'branch': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'user_id'
            ]
        },
        'global.ResponseRepositoryPush': {
            'type': 'object',
            'properties': {
                'msg': {
                    'type': 'string'
                },
                'file_id': {
                    'type': 'string'
                },
                'device_id': {
                    'type': 'string'
                }
            },
            'required': [
                'device_id',
                'file_id',
                'msg'
            ]
        },
        'global.RequestTargetCommonResponse': {
            'type': 'object',
            'properties': {
                'err': {
                    'type': 'boolean'
                },
                'status': {
                    'type': 'number'
                },
                'data': {
                    '$ref': '#/definitions/S'
                },
                'msg': {
                    'type': 'string'
                }
            },
            'required': [
                'err',
                'msg',
                'status'
            ]
        },
        'global.ResponseCommitInfo': {
            'type': 'object',
            'properties': {
                'commit': {
                    'type': 'string'
                },
                'author': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                },
                'message': {
                    'type': 'string'
                }
            },
            'required': [
                'author',
                'commit',
                'date',
                'message'
            ]
        },
        'global.ResponseRepositoryHome': {
            'type': 'object',
            'properties': {
                'repository': {
                    '$ref': '#/definitions/ResponseRepository'
                },
                'commitCount': {
                    'type': 'number'
                },
                'branches': {
                    'type': 'array',
                    'items': {
                        'type': 'string'
                    }
                },
                'lastCommit': {
                    'type': 'object',
                    'properties': {
                        'commit': {
                            'type': 'string'
                        },
                        'author': {
                            'type': 'string'
                        },
                        'date': {
                            'type': 'number'
                        },
                        'message': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'author',
                        'commit',
                        'date',
                        'message'
                    ]
                },
                'releaseCount': {
                    'type': 'number'
                }
            },
            'required': [
                'branches',
                'commitCount',
                'repository'
            ]
        },
        'global.RequestRepositoryDelivery': {
            'type': 'object',
            'properties': {
                'owner_id': {
                    'type': 'string'
                },
                'organization_id': {
                    'type': 'string'
                },
                'new_member_id': {
                    'type': 'string'
                },
                'new_member_name': {
                    'type': 'string'
                }
            },
            'required': [
                'new_member_id',
                'new_member_name',
                'organization_id',
                'owner_id'
            ]
        },
        'global.RequestRepositoryRemoteDeleteByOwner': {
            'type': 'object',
            'properties': {
                'repository_id': {
                    'type': 'string'
                }
            },
            'required': [
                'repository_id'
            ]
        },
        'global.RequestRepositoryStateSwitch': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'action': {
                    '$ref': '#/definitions/SetStateType'
                }
            },
            'required': [
                'action',
                'id',
                'owner'
            ]
        },
        'global.RepositoryFileMessage': {
            'type': 'object',
            'properties': {
                'file_data': {
                    'type': 'array',
                    'items': {
                        'allOf': [
                            {
                                'type': 'object',
                                'properties': {
                                    'commit': {
                                        'type': 'string'
                                    },
                                    'author': {
                                        'type': 'string'
                                    },
                                    'date': {
                                        'type': 'number'
                                    },
                                    'message': {
                                        'type': 'string'
                                    }
                                },
                                'required': [
                                    'author',
                                    'commit',
                                    'date',
                                    'message'
                                ]
                            },
                            {
                                'type': 'object',
                                'properties': {
                                    'fileType': {
                                        'type': 'string'
                                    },
                                    'file': {
                                        'type': 'string'
                                    }
                                },
                                'required': [
                                    'file',
                                    'fileType'
                                ]
                            }
                        ]
                    }
                },
                'readme': {
                    'type': 'object',
                    'properties': {
                        'content': {
                            'type': 'string'
                        },
                        'type': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'content',
                        'type'
                    ]
                }
            },
            'required': [
                'file_data'
            ]
        },
        'global.RepositoryFileReadme': {
            'type': 'object',
            'properties': {
                'content': {
                    'type': 'string'
                },
                'type': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'type'
            ]
        },
        'global.FileData': {
            'allOf': [
                {
                    'type': 'object',
                    'properties': {
                        'commit': {
                            'type': 'string'
                        },
                        'author': {
                            'type': 'string'
                        },
                        'date': {
                            'type': 'number'
                        },
                        'message': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'author',
                        'commit',
                        'date',
                        'message'
                    ]
                },
                {
                    'type': 'object',
                    'properties': {
                        'fileType': {
                            'type': 'string'
                        },
                        'file': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'file',
                        'fileType'
                    ]
                }
            ]
        },
        'global.ResponseRepositoryAnalysisItem': {
            'type': 'object',
            'properties': {
                'code': {
                    'type': 'number'
                },
                'type': {
                    'type': 'string'
                }
            },
            'required': [
                'code',
                'type'
            ]
        },
        'global.ResponseRepositoryAnalysis': {
            'type': 'object',
            'properties': {
                'data': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'code': {
                                'type': 'number'
                            },
                            'type': {
                                'type': 'string'
                            }
                        },
                        'required': [
                            'code',
                            'type'
                        ]
                    }
                }
            },
            'required': [
                'data'
            ]
        },
        'global.RequestRepositoryReleaseAdd': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                },
                'tag_name': {
                    'type': 'string'
                },
                'tag_target': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                },
                'content': {
                    'type': 'string'
                },
                'owner': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'id',
                'owner',
                'tag_name',
                'tag_target',
                'title',
                'user_id'
            ]
        },
        'global.RequestRepositoryReleaseDelete': {
            'type': 'object',
            'properties': {
                'release_id': {
                    'type': 'string'
                }
            },
            'required': [
                'release_id'
            ]
        },
        'global.RequestRepositoryReleaseEdit': {
            'type': 'object',
            'properties': {
                'tag_name': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                },
                'content': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'id',
                'tag_name',
                'title'
            ]
        },
        'global.RequestRepositoryReleaseDetail': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'tag_name': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'tag_name'
            ]
        },
        'global.RequestRepositoryReleaseDownload': {
            'type': 'object',
            'properties': {
                'file_id': {
                    'type': 'string'
                },
                'repo_owner': {
                    'type': 'string'
                },
                'repo_id': {
                    'type': 'string'
                },
                'repo_name': {
                    'type': 'string'
                },
                'tag_name': {
                    'type': 'string'
                }
            },
            'required': [
                'file_id',
                'repo_id',
                'repo_name',
                'repo_owner',
                'tag_name'
            ]
        },
        'global.ResponseRepositoryRelease': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'commit_id': {
                    'type': 'string'
                },
                'publisher_id': {
                    'type': 'string'
                },
                'tag_name': {
                    'type': 'string'
                },
                'tag_target': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                },
                'content': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                },
                'file_id': {
                    'type': 'string'
                }
            },
            'required': [
                'commit_id',
                'content',
                'date',
                'file_id',
                'id',
                'publisher_id',
                'tag_name',
                'tag_target',
                'title'
            ]
        },
        'global.ResponseRepoWikiHome': {
            'type': 'object',
            'properties': {
                'data': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'content': {
                                'type': 'string'
                            },
                            'date': {
                                'type': 'number'
                            },
                            'id': {
                                'type': 'string'
                            },
                            'publisher_id': {
                                'type': 'string'
                            },
                            'title': {
                                'type': 'string'
                            }
                        },
                        'required': [
                            'content',
                            'date',
                            'id',
                            'publisher_id',
                            'title'
                        ]
                    }
                },
                'page': {
                    'type': 'object',
                    'properties': {
                        'content': {
                            'type': 'string'
                        },
                        'date': {
                            'type': 'number'
                        },
                        'id': {
                            'type': 'string'
                        },
                        'publisher_id': {
                            'type': 'string'
                        },
                        'title': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'content',
                        'date',
                        'id',
                        'publisher_id',
                        'title'
                    ]
                }
            },
            'required': [
                'data',
                'page'
            ]
        },
        'global.ResponseRepoWikiPage': {
            'type': 'object',
            'properties': {
                'content': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                },
                'id': {
                    'type': 'string'
                },
                'publisher_id': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'date',
                'id',
                'publisher_id',
                'title'
            ]
        },
        'global.RequestWikiPageHome': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                }
            },
            'required': [
                'id'
            ]
        },
        'global.RequestRepositoryWikiPageList': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'id'
            ]
        },
        'global.RequestRepositoryWikiPageNew': {
            'type': 'object',
            'properties': {
                'user_id': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                },
                'content': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'id',
                'title',
                'user_id'
            ]
        },
        'global.RequestRepositoryWikiDetail': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'title'
            ]
        },
        'global.RequestRepositoryWikiEdit': {
            'type': 'object',
            'properties': {
                'title': {
                    'type': 'string'
                },
                'content': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'wiki_id': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'id',
                'title',
                'wiki_id'
            ]
        },
        'global.RequestRepositoryWikiDelete': {
            'type': 'object',
            'properties': {
                'wiki_id': {
                    'type': 'string'
                }
            },
            'required': [
                'wiki_id'
            ]
        },
        'global.RequestRepositoryHome': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'branch': {
                    'type': 'string'
                }
            },
            'required': [
                'branch',
                'id'
            ]
        },
        'global.RequestRepositoryFork': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                },
                'ood': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'ood',
                'user_id'
            ]
        },
        'global.ResponseRepositoryStar': {
            'type': 'object',
            'properties': {
                'number': {
                    'type': 'number'
                },
                'forkNumber': {
                    'type': 'number'
                },
                'stared': {
                    'type': 'boolean'
                }
            },
            'required': [
                'forkNumber',
                'number',
                'stared'
            ]
        },
        'global.ResponseFile': {
            'type': 'object',
            'properties': {
                'fileType': {
                    'type': 'string'
                },
                'fileData': {
                    'type': 'object',
                    'properties': {
                        'content': {
                            'type': 'array',
                            'items': {
                                'type': 'object',
                                'properties': {
                                    'line': {
                                        'type': 'number'
                                    },
                                    'content': {
                                        'type': 'string'
                                    }
                                },
                                'required': [
                                    'content',
                                    'line'
                                ]
                            }
                        },
                        'bigFile': {
                            'type': 'boolean'
                        },
                        'notSupport': {
                            'type': 'boolean'
                        },
                        'info': {
                            'allOf': [
                                {
                                    'type': 'object',
                                    'properties': {
                                        'commit': {
                                            'type': 'string'
                                        },
                                        'author': {
                                            'type': 'string'
                                        },
                                        'date': {
                                            'type': 'number'
                                        },
                                        'message': {
                                            'type': 'string'
                                        }
                                    },
                                    'required': [
                                        'author',
                                        'commit',
                                        'date',
                                        'message'
                                    ]
                                },
                                {
                                    'type': 'object',
                                    'properties': {
                                        'fileSize': {
                                            'type': 'number'
                                        }
                                    },
                                    'required': [
                                        'fileSize'
                                    ]
                                }
                            ]
                        }
                    },
                    'required': [
                        'bigFile',
                        'content',
                        'info',
                        'notSupport'
                    ]
                },
                'dirData': {
                    'type': 'object',
                    'properties': {
                        'data': {
                            'type': 'array',
                            'items': {
                                'allOf': [
                                    {
                                        'type': 'object',
                                        'properties': {
                                            'commit': {
                                                'type': 'string'
                                            },
                                            'author': {
                                                'type': 'string'
                                            },
                                            'date': {
                                                'type': 'number'
                                            },
                                            'message': {
                                                'type': 'string'
                                            }
                                        },
                                        'required': [
                                            'author',
                                            'commit',
                                            'date',
                                            'message'
                                        ]
                                    },
                                    {
                                        'type': 'object',
                                        'properties': {
                                            'fileType': {
                                                'type': 'string'
                                            },
                                            'file': {
                                                'type': 'string'
                                            }
                                        },
                                        'required': [
                                            'file',
                                            'fileType'
                                        ]
                                    }
                                ]
                            }
                        },
                        'readme': {
                            'type': 'object',
                            'properties': {
                                'content': {
                                    'type': 'string'
                                },
                                'type': {
                                    'type': 'string'
                                }
                            },
                            'required': [
                                'content',
                                'type'
                            ]
                        }
                    },
                    'required': [
                        'data'
                    ]
                }
            },
            'required': [
                'fileType'
            ]
        },
        'global.ResponseCompareFile': {
            'type': 'object',
            'properties': {
                'notSupport': {
                    'type': 'boolean'
                },
                'content': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'pathName': {
                                'type': 'string'
                            },
                            'title': {
                                'type': 'string'
                            },
                            'data': {
                                'type': 'array',
                                'items': {
                                    'type': 'string'
                                }
                            }
                        },
                        'required': [
                            'data',
                            'pathName',
                            'title'
                        ]
                    }
                }
            },
            'required': [
                'content',
                'notSupport'
            ]
        },
        'global.CompareCommit': {
            'type': 'object',
            'properties': {
                'commit': {
                    'type': 'string'
                },
                'message': {
                    'type': 'string'
                }
            },
            'required': [
                'commit',
                'message'
            ]
        },
        'global.ResponseMergeCompare': {
            'type': 'object',
            'properties': {
                'commits': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'commit': {
                                'type': 'string'
                            },
                            'message': {
                                'type': 'string'
                            }
                        },
                        'required': [
                            'commit',
                            'message'
                        ]
                    }
                },
                'diff': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'fileName': {
                                'type': 'string'
                            },
                            'diffType': {
                                'type': 'string'
                            },
                            'count': {
                                'type': 'string'
                            }
                        },
                        'required': [
                            'count',
                            'fileName'
                        ]
                    }
                }
            },
            'required': [
                'commits',
                'diff'
            ]
        },
        'global.RequestRepositoryList': {
            'type': 'object',
            'properties': {
                'repo_name': {
                    'type': 'string'
                },
                'page_index': {
                    'type': 'number'
                },
                'page_size': {
                    'type': 'number'
                }
            }
        },
        'global.RequestRepositoryFind': {
            'type': 'object',
            'properties': {
                'repo_name': {
                    'type': 'string'
                },
                'author_name': {
                    'type': 'string'
                }
            },
            'required': [
                'author_name',
                'repo_name'
            ]
        },
        'global.RequestRepositoryBranches': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'owner'
            ]
        },
        'global.RequestRepositoryDelete': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'owner'
            ]
        },
        'global.RequestRepositoryDrop': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'user_id'
            ]
        },
        'global.RequestRepositoryMerges': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'owner'
            ]
        },
        'global.RequestRepositoryReleaseList': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'id'
            ]
        },
        'global.RequestRepositoryStarStatus': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'user_id'
            ]
        },
        'global.RequestRepositoryStarAdd': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'user_id'
            ]
        },
        'global.RequestRepositoryStarDelete': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'user_id'
            ]
        },
        'global.RequestOrganizationByName': {
            'type': 'object',
            'properties': {
                'organization_name': {
                    'type': 'string'
                }
            },
            'required': [
                'organization_name'
            ]
        },
        'global.RequestOrganizationList': {
            'type': 'object',
            'properties': {
                'organization_name': {
                    'type': 'string'
                },
                'page_index': {
                    'type': 'number'
                },
                'page_size': {
                    'type': 'number'
                }
            }
        },
        'global.RequestSerivceRepositoryList': {
            'type': 'object',
            'properties': {
                'repo_name': {
                    'type': 'string'
                },
                'page_index': {
                    'type': 'number'
                },
                'page_size': {
                    'type': 'number'
                },
                'user_id': {
                    'type': 'string'
                }
            }
        },
        'global.RequestUserList': {
            'type': 'object',
            'properties': {
                'user_name': {
                    'type': 'string'
                },
                'page_index': {
                    'type': 'number'
                },
                'page_size': {
                    'type': 'number'
                }
            }
        },
        'global.RequestGetIdByName': {
            'type': 'object',
            'properties': {
                'user_name': {
                    'type': 'string'
                }
            },
            'required': [
                'user_name'
            ]
        },
        'global.RepositoryFindRequest': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                }
            },
            'required': [
                'name',
                'owner'
            ]
        },
        'requestFileData': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'file_name': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'branch': {
                    'type': 'string'
                }
            },
            'required': [
                'branch',
                'file_name',
                'id'
            ]
        },
        'ResponseCommit': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'oid': {
                    'type': 'string'
                },
                'message': {
                    'type': 'string'
                },
                'parent': {
                    'type': 'string'
                },
                'author': {},
                'date': {
                    'type': 'number'
                }
            },
            'required': [
                'author',
                'date',
                'id',
                'message',
                'oid',
                'owner',
                'parent'
            ]
        },
        'RequestCommits': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'branch': {
                    'type': 'string'
                }
            },
            'required': [
                'branch',
                'id',
                'owner'
            ]
        },
        'responseUser': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                },
                'email': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                }
            },
            'required': [
                'date',
                'email',
                'id',
                'name'
            ]
        },
        'ResponseIssue': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                },
                'repo_id': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                },
                'content': {
                    'type': 'string'
                },
                'status': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                },
                'issues': {
                    'type': 'array',
                    'items': {
                        '$ref': '#/definitions/ResponseIssue'
                    }
                },
                'commentLength': {
                    'type': 'number'
                }
            },
            'required': [
                'content',
                'date',
                'id',
                'status',
                'title',
                'user_id'
            ]
        },
        'ResponseIssueList': {
            'type': 'object',
            'properties': {
                'list': {
                    'type': 'array',
                    'items': {
                        '$ref': '#/definitions/ResponseIssue'
                    }
                },
                'open': {
                    'type': 'number'
                },
                'close': {
                    'type': 'number'
                },
                'mine': {
                    'type': 'number'
                },
                'other': {
                    'type': 'number'
                }
            },
            'required': [
                'close',
                'list',
                'mine',
                'open',
                'other'
            ]
        },
        'ResponseMerge': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                },
                'repo_id': {
                    'type': 'string'
                },
                'repository_owner_id': {
                    'type': 'string'
                },
                'originBranch': {
                    'type': 'string'
                },
                'targetBranch': {
                    'type': 'string'
                },
                'mergeType': {
                    'type': 'string'
                },
                'status': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                }
            },
            'required': [
                'date',
                'id',
                'mergeType',
                'originBranch',
                'repo_id',
                'repository_owner_id',
                'status',
                'targetBranch',
                'title',
                'user_id'
            ]
        },
        'ResponseMergeList': {
            'type': 'object',
            'properties': {
                'list': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'id': {
                                'type': 'string'
                            },
                            'title': {
                                'type': 'string'
                            },
                            'user_id': {
                                'type': 'string'
                            },
                            'repo_id': {
                                'type': 'string'
                            },
                            'repository_owner_id': {
                                'type': 'string'
                            },
                            'originBranch': {
                                'type': 'string'
                            },
                            'targetBranch': {
                                'type': 'string'
                            },
                            'mergeType': {
                                'type': 'string'
                            },
                            'status': {
                                'type': 'string'
                            },
                            'date': {
                                'type': 'number'
                            }
                        },
                        'required': [
                            'date',
                            'id',
                            'mergeType',
                            'originBranch',
                            'repo_id',
                            'repository_owner_id',
                            'status',
                            'targetBranch',
                            'title',
                            'user_id'
                        ]
                    }
                },
                'open': {
                    'type': 'number'
                },
                'close': {
                    'type': 'number'
                },
                'mine': {
                    'type': 'number'
                },
                'other': {
                    'type': 'number'
                }
            },
            'required': [
                'close',
                'list',
                'mine',
                'open',
                'other'
            ]
        },
        'RequestRepositoryMergeCompare': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'target': {
                    'type': 'string'
                },
                'origin': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'origin',
                'target'
            ]
        },
        'RequestRepositoryMergeCreate': {
            'type': 'object',
            'properties': {
                'repository_owner_id': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'target': {
                    'type': 'string'
                },
                'origin': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                },
                'mergeType': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'mergeType',
                'origin',
                'repository_owner_id',
                'target',
                'title'
            ]
        },
        'RequestRepositoryMergeCompareFile': {
            'allOf': [
                {
                    'type': 'object',
                    'properties': {
                        'owner': {
                            'type': 'string'
                        },
                        'id': {
                            'type': 'string'
                        },
                        'target': {
                            'type': 'string'
                        },
                        'origin': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'id',
                        'origin',
                        'target'
                    ]
                },
                {
                    'type': 'object',
                    'properties': {
                        'fileName': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'fileName'
                    ]
                }
            ]
        },
        'RequestRepositoryMergeDetail': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'merge_id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'merge_id',
                'owner'
            ]
        },
        'RequestRepositoryCommit': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'commitId': {
                    'type': 'string'
                }
            },
            'required': [
                'commitId',
                'id'
            ]
        },
        'RequestIssueCreate': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                },
                'content': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'id',
                'owner',
                'title',
                'user_id'
            ]
        },
        'RequestIssueDetail': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'issue_id': {
                    'type': 'string'
                }
            },
            'required': [
                'issue_id'
            ]
        },
        'RequestRepoIssue': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                }
            },
            'required': [
                'id'
            ]
        },
        'RequestUserInit': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                },
                'email': {
                    'type': 'string'
                }
            },
            'required': [
                'email',
                'name',
                'owner'
            ]
        },
        'RequestOrganizationCreate': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                },
                'email': {
                    'type': 'string'
                }
            },
            'required': [
                'email',
                'name',
                'owner'
            ]
        },
        'RequestRepositoryPush': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'binary': {
                    'type': 'string'
                },
                'runtimeDeviceId': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'binary',
                'id',
                'runtimeDeviceId',
                'user_id'
            ]
        },
        'RequestRepositoryPushV2': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'packFileId': {
                    'type': 'string'
                },
                'refs': {
                    'type': 'string'
                },
                'runtimeDeviceId': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'packFileId',
                'refs',
                'runtimeDeviceId',
                'user_id'
            ]
        },
        'RequestRepositoryPushHead': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'branch': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'id',
                'user_id'
            ]
        },
        'RequestRepositoryFetch': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'hash': {
                    'type': 'string'
                },
                'localHash': {
                    'type': 'string'
                },
                'ref': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'hash',
                'id',
                'localHash',
                'ref',
                'user_id'
            ]
        },
        'RequestRepositoryTrans': {
            'type': 'object',
            'properties': {
                'id': {
                    'type': 'string'
                },
                'binary': {
                    'type': 'string'
                },
                'runtimeDeviceId': {
                    'type': 'string'
                },
                'target': {
                    'type': 'string'
                }
            },
            'required': [
                'binary',
                'id',
                'runtimeDeviceId',
                'target'
            ]
        },
        'RequestUserSetting': {
            'type': 'object',
            'properties': {
                'userId': {
                    'type': 'string'
                },
                'owner': {
                    'type': 'string'
                },
                'email': {
                    'type': 'string'
                }
            },
            'required': [
                'email',
                'userId'
            ]
        },
        'ResponseFileCommit': {
            'type': 'object',
            'properties': {
                'commit': {
                    'type': 'string'
                },
                'author': {
                    'type': 'string'
                },
                'date': {
                    'type': 'number'
                },
                'message': {
                    'type': 'string'
                }
            },
            'required': [
                'author',
                'commit',
                'date',
                'message'
            ]
        },
        'ResponseFileLineContent': {
            'type': 'object',
            'properties': {
                'line': {
                    'type': 'number'
                },
                'content': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'line'
            ]
        },
        'ResponseCommitShowDiff': {
            'type': 'object',
            'properties': {
                'data': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'pathName': {
                                'type': 'string'
                            },
                            'title': {
                                'type': 'string'
                            },
                            'data': {
                                'type': 'array',
                                'items': {
                                    'type': 'string'
                                }
                            }
                        },
                        'required': [
                            'data',
                            'pathName',
                            'title'
                        ]
                    }
                },
                'commitId': {
                    'type': 'string'
                },
                'headerInfo': {
                    'type': 'object',
                    'properties': {
                        'commit': {
                            'type': 'string'
                        },
                        'author': {
                            'type': 'string'
                        },
                        'date': {
                            'type': 'number'
                        },
                        'message': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'author',
                        'commit',
                        'date',
                        'message'
                    ]
                }
            },
            'required': [
                'commitId',
                'data',
                'headerInfo'
            ]
        },
        'ResponseFineRepository': {
            'type': 'object',
            'properties': {
                'repo': {
                    'type': 'string'
                },
                'device': {
                    'type': 'string'
                }
            },
            'required': [
                'device',
                'repo'
            ]
        },
        'ResponseAddFile': {
            'type': 'object',
            'properties': {
                'err': {
                    'type': 'boolean'
                },
                'msg': {
                    'type': 'string'
                },
                'file_id': {
                    'type': 'string'
                }
            },
            'required': [
                'err',
                'msg'
            ]
        },
        'ResponseCheckUser': {
            'type': 'object',
            'properties': {
                'userInit': {
                    'type': 'boolean'
                },
                'user': {
                    'type': 'object',
                    'properties': {
                        'userId': {
                            'type': 'string'
                        },
                        'name': {
                            'type': 'string'
                        },
                        'email': {
                            'type': 'string'
                        },
                        'date': {
                            'type': 'number'
                        },
                        'owner': {
                            'type': 'string'
                        }
                    },
                    'required': [
                        'date',
                        'email',
                        'name',
                        'userId'
                    ]
                }
            },
            'required': [
                'userInit'
            ]
        },
        'GetObjectResponse': {
            'type': 'object',
            'properties': {
                'err': {
                    'type': 'boolean'
                },
                'object': {
                    '$ref': '#/definitions/S_1'
                },
                'message': {
                    'type': 'string'
                }
            },
            'required': [
                'err',
                'message'
            ]
        },
        'RequestRepoIssueComment': {
            'type': 'object',
            'properties': {
                'issue_id': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'owner': {
                    'type': 'string'
                },
                'content': {
                    'type': 'string'
                },
                'user_id': {
                    'type': 'string'
                }
            },
            'required': [
                'content',
                'issue_id',
                'owner',
                'user_id'
            ]
        },
        'DiffResult': {
            'type': 'object',
            'properties': {
                'fileName': {
                    'type': 'string'
                },
                'diffType': {
                    'type': 'string'
                },
                'count': {
                    'type': 'string'
                }
            },
            'required': [
                'count',
                'fileName'
            ]
        },
        'DiffLineData': {
            'type': 'object',
            'properties': {
                'pathName': {
                    'type': 'string'
                },
                'title': {
                    'type': 'string'
                },
                'data': {
                    'type': 'array',
                    'items': {
                        'type': 'string'
                    }
                }
            },
            'required': [
                'data',
                'pathName',
                'title'
            ]
        },
        'DiffParseResponse': {
            'type': 'object',
            'properties': {
                'data': {
                    'type': 'array',
                    'items': {
                        'type': 'object',
                        'properties': {
                            'pathName': {
                                'type': 'string'
                            },
                            'title': {
                                'type': 'string'
                            },
                            'data': {
                                'type': 'array',
                                'items': {
                                    'type': 'string'
                                }
                            }
                        },
                        'required': [
                            'data',
                            'pathName',
                            'title'
                        ]
                    }
                }
            },
            'required': [
                'data'
            ]
        },
        'ResponseRepository': {
            'type': 'object',
            'properties': {
                'owner': {
                    'type': 'string'
                },
                'id': {
                    'type': 'string'
                },
                'name': {
                    'type': 'string'
                },
                'description': {
                    'type': 'string'
                },
                'is_private': {
                    '$ref': '#/definitions/RepositoryType'
                },
                'init': {
                    'type': 'number'
                },
                'binary_id': {
                    'type': 'string'
                },
                'fork_from': {
                    'type': 'string'
                },
                'author_type': {
                    'type': 'string'
                },
                'author_name': {
                    'type': 'string'
                },
                'releaseCount': {
                    'type': 'number'
                },
                'fork_repository': {
                    '$ref': '#/definitions/ResponseRepository'
                },
                'date': {
                    'type': 'number'
                }
            },
            'required': [
                'author_name',
                'author_type',
                'binary_id',
                'description',
                'fork_from',
                'id',
                'init',
                'is_private',
                'name',
                'owner'
            ]
        },
        'S': {
            'type': 'object'
        },
        'S_1': {
            'type': 'object'
        }
    }
};