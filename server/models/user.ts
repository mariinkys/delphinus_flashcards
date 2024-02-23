import { User } from "@prisma/client";

export type UserModel = {
    id: number | null,
    username: string,
    email: string,
    password: string,
    createdAt: Date,
    updatedAt: Date,
}

export function mapPrismaUserModel(prismaModel: User): UserModel {
    return {
        id: prismaModel.id,
        username: prismaModel.username,
        email: prismaModel.email,
        password: prismaModel.password,
        createdAt: prismaModel.createdAt,
        updatedAt: prismaModel.updatedAt
    };
}

export function initDefaultUser(): UserModel {
    return {
        id: null,
        username: "",
        email: "",
        password: "",
        createdAt: new Date,
        updatedAt: new Date
    };
}