import { getServerSession } from '#auth';
import { Prisma } from '@prisma/client';
import bcrypt from 'bcryptjs';
import prisma from './../db/prisma';

export default defineEventHandler(async (event) => {
    const data = await readBody(event)
    const body = data.values
    if (validateRegisterBody(body)) {
        try {
            const hash = bcrypt.hashSync(body.password, 12);
            await prisma.user.create({
                data: {
                    username: body.username,
                    email: body.email,
                    password: hash
                },
            })
            setResponseStatus(event, 200)
            return event
        } catch (error) {
            if (error instanceof Prisma.PrismaClientKnownRequestError) {
                // The .code property can be accessed in a type-safe manner
                if (error.code === 'P2002') {
                    console.log(
                        'Username or email already taken!'
                    )
                    return null
                }
            }
            setResponseStatus(event, 404)
            return event
        }
    } else {
        setResponseStatus(event, 404)
        return event
    }
})

function validateRegisterBody(body: any): boolean {
    if (body.username.length <= 0 || body.username == null) {
        return false
    }
    if (body.password.length <= 0 || body.password == null) {
        return false
    }
    if (body.repeat_password.length <= 0 || body.repeat_password == null) {
        return false
    }
    if (body.password !== body.repeat_password) {
        return false
    }
    return true
}