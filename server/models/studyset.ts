import { StudySet, User } from "@prisma/client";

export type StudySetModel = {
    id: number | null,
    name: string,
    pertainsTo: User | null,
    userId: number | null,
    createdAt: Date,
    updatedAt: Date,
}

export function mapPrismaStudySetModel(prismaModel: StudySet): StudySetModel {
    return {
        id: prismaModel.id,
        name: prismaModel.name,
        pertainsTo: null,
        userId: prismaModel.userId,
        createdAt: prismaModel.createdAt,
        updatedAt: prismaModel.updatedAt
    };
}

export function initDefaultStudySet(): StudySetModel {
    return {
        id: null,
        name: "",
        pertainsTo: null,
        userId: null,
        createdAt: new Date,
        updatedAt: new Date
    };
}