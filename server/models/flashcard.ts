import { Flashcard, Folder } from "@prisma/client";

export type FlashcardModel = {
    id: number | null,
    front: string,
    back: string,
    flashcardStatus: number,
    insideOf: Folder | null,
    folderId: number | null,
    createdAt: Date,
    updatedAt: Date,
}

export function mapPrismaFlashcardModel(prismaModel: Flashcard): FlashcardModel {
    return {
        id: prismaModel.id,
        front: prismaModel.front,
        back: prismaModel.back,
        flashcardStatus: prismaModel.flashcardStatus,
        insideOf: null,
        folderId: prismaModel.folderId,
        createdAt: prismaModel.createdAt,
        updatedAt: prismaModel.updatedAt
    };
}

export function initDefaultFlashcard(): FlashcardModel {
    return {
        id: null,
        front: "",
        back: "",
        flashcardStatus: 0,
        insideOf: null,
        folderId: null,
        createdAt: new Date,
        updatedAt: new Date
    };
}