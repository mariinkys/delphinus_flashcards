import { Folder, StudySet, User } from "@prisma/client";

export type FolderModel = {
    id: number | null,
    name: string,
    pertainsTo: User | null,
    userId: number | null,
    StudySet: StudySet | null,
    studySetId: number | null,
    createdAt: Date,
    updatedAt: Date,
}

export function mapPrismaFolderModel(prismaModel: Folder): FolderModel {
    return {
        id: prismaModel.id,
        name: prismaModel.name,
        pertainsTo: null,
        userId: prismaModel.userId,
        StudySet: null,
        studySetId: prismaModel.studySetId,
        createdAt: prismaModel.createdAt,
        updatedAt: prismaModel.updatedAt
    };
}

export function initDefaultFolder(): FolderModel {
    return {
        id: null,
        name: "",
        pertainsTo: null,
        userId: null,
        studySetId: null,
        StudySet: null,
        createdAt: new Date,
        updatedAt: new Date
    };
}