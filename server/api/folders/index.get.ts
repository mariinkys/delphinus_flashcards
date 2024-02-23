import { getServerSession, getToken } from '#auth';
import prisma from './../../db/prisma';

export default defineEventHandler(async (event) => {
    const session = await getServerSession(event)
    if (!session) {
        setResponseStatus(event, 404)
        return event
    }

    const token = await getToken({ event })

    if (token?.sub) {
        try {
            const folders = await prisma.folder.findMany({
                where: {
                    userId: +token.sub
                }
            });
            setResponseStatus(event, 200)
            return folders
        } catch (error) {
            setResponseStatus(event, 404)
            return event
        }
    } else {
        setResponseStatus(event, 404)
        return event
    }
})