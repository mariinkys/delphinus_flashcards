import { getServerSession, getToken } from '#auth';
import prisma from './../../db/prisma';

export default defineEventHandler(async (event) => {
    const session = await getServerSession(event)
    if (!session) {
        setResponseStatus(event, 404)
        return event
    }

    const token = await getToken({ event })

    const data = await readBody(event)

    if (token?.sub) {
        try {
            await prisma.folder.create({
                data: {
                    name: data.folder.name,
                    studySetId: +data.folder.studySetId,
                    userId: +token.sub
                }
            });
            setResponseStatus(event, 200)
            return event
        } catch (error) {
            setResponseStatus(event, 404)
            return event
        }
    } else {
        setResponseStatus(event, 404)
        return event
    }
})